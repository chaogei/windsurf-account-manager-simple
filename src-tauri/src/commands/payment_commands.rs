use tauri::{command, AppHandle, WebviewWindowBuilder, WebviewUrl, Manager, Listener, State};
use std::process::Command;
use std::sync::Arc;
use crate::utils::card_generator::{CardGenerator, VirtualCard};
use crate::repository::DataStore;
use serde_json::json;
use std::fs;
use uuid::Uuid;

#[command]
pub async fn generate_virtual_card(data_store: State<'_, Arc<DataStore>>) -> Result<VirtualCard, String> {
    // 获取设置中的自定义卡头和卡段范围
    let settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
    let custom_bin = settings.custom_card_bin;
    let bin_range = settings.custom_card_bin_range;
    
    // 使用自定义卡头或卡段范围生成虚拟卡
    Ok(CardGenerator::generate_card_with_bin_or_range(&custom_bin, bin_range.as_deref()))
}

#[command]
pub async fn open_payment_window(
    app: AppHandle,
    url: String,
    account_name: String,
) -> Result<String, String> {
    let window_label = format!("payment-{}", chrono::Utc::now().timestamp_millis());
    let window_title = format!("Stripe 支付页面 - {} (隐私模式)", account_name);
    
    // 创建临时的用户数据目录（模拟Chrome的无痕模式）
    let temp_dir = std::env::temp_dir();
    let session_id = Uuid::new_v4().to_string();
    let user_data_dir = temp_dir.join(format!("windsurf_incognito_{}", session_id));
    
    // 确保目录存在
    if !user_data_dir.exists() {
        fs::create_dir_all(&user_data_dir).map_err(|e| e.to_string())?;
    }
    
    println!("[Incognito] 创建临时用户数据目录: {:?}", user_data_dir);
    
    // 创建新的webview窗口（Chrome风格的无痕模式）
    let window_builder = WebviewWindowBuilder::new(
        &app,
        window_label.clone(),
        WebviewUrl::External(url.parse().unwrap())
    )
    .title(window_title)
    .inner_size(1200.0, 800.0)
    .resizable(true)
    .minimizable(true)
    .closable(true)
    .center()
    .incognito(true)  // 启用无痕模式
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.130 Safari/537.36");  // Chrome 120最新版UA
    
    // 设置更多隐私相关的WebView选项
    #[cfg(target_os = "windows")]  
    {
        // Windows特定：使用临时用户数据文件夹
        window_builder = window_builder
            .data_directory(user_data_dir.clone());  // 设置独立的数据目录
    }
    
    let window = window_builder.build()
        .map_err(|e: tauri::Error| e.to_string())?;
    
    // 注入Chrome无痕模式风格的隐私保护脚本
    let anti_fingerprint_script = r#"
        // Chrome Incognito Mode Privacy Protection
        (function() {
            'use strict';
            
            console.log('[Chrome Incognito] Privacy protection script loaded');
            
            // 1. 模拟Chrome无痕模式的API行为
            // 禁用本地存储跟踪
            const throwQuotaExceeded = () => {
                throw new DOMException('The quota has been exceeded.', 'QuotaExceededError');
            };
            
            // 限制localStorage和sessionStorage
            try {
                window.localStorage.setItem = throwQuotaExceeded;
                window.sessionStorage.setItem = throwQuotaExceeded;
            } catch (e) {}
            
            // 2. 阻止WebRTC IP泄露
            const noop = () => {};
            const rtcBlocked = {
                createDataChannel: noop,
                createOffer: () => Promise.reject(new Error('WebRTC blocked')),
                createAnswer: () => Promise.reject(new Error('WebRTC blocked')),
                setLocalDescription: noop,
                setRemoteDescription: noop,
                addIceCandidate: noop,
                getStats: () => Promise.resolve(new Map()),
                close: noop
            };
            
            if (window.RTCPeerConnection) {
                window.RTCPeerConnection = function() { return rtcBlocked; };
                window.RTCPeerConnection.prototype = rtcBlocked;
            }
            if (window.webkitRTCPeerConnection) {
                window.webkitRTCPeerConnection = function() { return rtcBlocked; };
            }
            
            // 3. Canvas指纹防护（Chrome风格）
            const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
            const originalToBlob = HTMLCanvasElement.prototype.toBlob;
            const originalGetImageData = CanvasRenderingContext2D.prototype.getImageData;
            
            const addNoise = (canvas, context) => {
                const width = canvas.width;
                const height = canvas.height;
                const imageData = originalGetImageData.call(context, 0, 0, width, height);
                
                // 添加极其微小的噪声，不影响视觉效果
                for (let i = 0; i < imageData.data.length; i += 4) {
                    const noise = (Math.random() - 0.5) * 0.01;
                    imageData.data[i] = Math.min(255, Math.max(0, imageData.data[i] + noise));
                    imageData.data[i + 1] = Math.min(255, Math.max(0, imageData.data[i + 1] + noise));
                    imageData.data[i + 2] = Math.min(255, Math.max(0, imageData.data[i + 2] + noise));
                }
                return imageData;
            };
            
            HTMLCanvasElement.prototype.toDataURL = function(...args) {
                const context = this.getContext('2d');
                if (context) {
                    const imageData = addNoise(this, context);
                    context.putImageData(imageData, 0, 0);
                }
                return originalToDataURL.apply(this, args);
            };
            
            HTMLCanvasElement.prototype.toBlob = function(callback, ...args) {
                const context = this.getContext('2d');
                if (context) {
                    const imageData = addNoise(this, context);
                    context.putImageData(imageData, 0, 0);
                }
                return originalToBlob.call(this, callback, ...args);
            };
            
            // 4. 硬件和设备信息伪装（Chrome标准值）
            Object.defineProperties(navigator, {
                hardwareConcurrency: { get: () => 8 },
                deviceMemory: { get: () => 8 },
                platform: { get: () => 'Win32' },
                vendor: { get: () => 'Google Inc.' },
                appVersion: { get: () => '5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.130 Safari/537.36' }
            });
            
            // 5. WebGL指纹防护
            const getParameterProxyHandler = {
                apply: function(target, thisArg, argumentsList) {
                    const parameter = argumentsList[0];
                    const originalValue = target.apply(thisArg, argumentsList);
                    
                    // 返回通用硬件信息
                    if (parameter === 37445) return 'Intel Inc.'; // UNMASKED_VENDOR_WEBGL
                    if (parameter === 37446) return 'Intel Iris OpenGL Engine'; // UNMASKED_RENDERER_WEBGL
                    
                    return originalValue;
                }
            };
            
            // 应用WebGL保护
            const hookWebGLGetParameter = (context) => {
                if (context.getParameter) {
                    context.getParameter = new Proxy(context.getParameter, getParameterProxyHandler);
                }
            };
            
            // Hook WebGL 上下文创建
            const originalGetContext = HTMLCanvasElement.prototype.getContext;
            HTMLCanvasElement.prototype.getContext = function(type, ...args) {
                const context = originalGetContext.call(this, type, ...args);
                if (type === 'webgl' || type === 'webgl2' || type === 'experimental-webgl') {
                    hookWebGLGetParameter(context);
                }
                return context;
            };
            
            // 6. 时区和语言伪装
            Object.defineProperty(Date.prototype, 'getTimezoneOffset', {
                value: function() { return -480; } // UTC+8
            });
            
            Object.defineProperty(navigator, 'language', {
                get: () => 'zh-CN'
            });
            
            Object.defineProperty(navigator, 'languages', {
                get: () => ['zh-CN', 'zh', 'en-US', 'en']
            });
            
            // 7. 禁用持久化存储API
            if (navigator.storage && navigator.storage.persist) {
                navigator.storage.persist = () => Promise.resolve(false);
            }
            
            if (navigator.storage && navigator.storage.estimate) {
                navigator.storage.estimate = () => Promise.resolve({
                    quota: 1073741824, // 1GB
                    usage: 0
                });
            }
            
            // 8. 禁用通知API
            if (window.Notification) {
                window.Notification.permission = 'denied';
                window.Notification.requestPermission = () => Promise.resolve('denied');
            }
            
            // 9. 添加Chrome无痕模式标识
            Object.defineProperty(window, 'chrome', {
                get: () => {
                    return {
                        ...window.chrome,
                        runtime: {
                            ...window.chrome?.runtime,
                            inIncognitoContext: true
                        }
                    };
                }
            });
            
            console.log('[Chrome Incognito] All privacy protections activated');
        })();
    "#;
    
    // 立即注入和延迟注入结合
    window.eval(anti_fingerprint_script).unwrap_or_else(|e| {
        println!("[Incognito] 初次注入失败: {}", e);
    });
    
    // 延迟再次注入确保生效
    let window_clone = window.clone();
    let script_clone = anti_fingerprint_script.to_string();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));
        let _ = window_clone.eval(&script_clone);
    });
    
    // 添加窗口关闭事件监听，清理临时文件
    let window_label_clone = window_label.clone();
    let user_data_dir_clone = user_data_dir.clone();
    window.once("tauri://close-requested", move |_| {
        println!("[Incognito] 窗口关闭: {}", window_label_clone);
        
        // 异步清理临时目录
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500)); // 等待窗口完全关闭
            
            if user_data_dir_clone.exists() {
                match fs::remove_dir_all(&user_data_dir_clone) {
                    Ok(_) => println!("[Incognito] 临时数据已清理: {:?}", user_data_dir_clone),
                    Err(e) => println!("[Incognito] 清理临时数据失败: {}", e),
                }
            }
        });
    });
    
    Ok(window_label)
}

#[command]
pub async fn inject_card_info(
    app: AppHandle,
    data_store: State<'_, Arc<DataStore>>,
    window_label: String,
    card_info: VirtualCard,
) -> Result<(), String> {
    // 获取设置中的卡段范围配置
    let settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
    let custom_bin = settings.custom_card_bin;
    let card_bin_range = settings.custom_card_bin_range;
    
    // 获取本地BIN池
    let local_bin_pool = if settings.use_local_success_bins {
        get_success_bins(app.clone()).await.unwrap_or_default()
    } else {
        vec![]
    };
    
    inject_card_info_internal(app, data_store.inner().clone(), window_label, card_info, card_bin_range, custom_bin, settings.card_bind_retry_times, settings.test_mode_enabled, settings.use_local_success_bins, local_bin_pool).await
}

/// 内部实现：注入卡信息到支付页面
async fn inject_card_info_internal(
    app: AppHandle,
    data_store: Arc<DataStore>,
    window_label: String,
    card_info: VirtualCard,
    card_bin_range: Option<String>,
    custom_bin: String,
    max_retries: i32,
    test_mode_enabled: bool,
    use_local_bin_pool: bool,
    local_bin_pool: Vec<String>,  // 本地BIN池内容（用于JS重试时随机选择）
) -> Result<(), String> {
    // 构建JavaScript代码来填写表单
    // 获取窗口
    let window = app.get_webview_window(&window_label)
        .ok_or("Window not found".to_string())?;
    
    // 稍微等待窗口稳定
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let js_code = format!(r#"
        (function() {{
            console.log('[AutoFill] 脚本已注入，开始执行...');
            
            // 快速填充 - React兼容版本
            function simulateTyping(element, value) {{
                if (!element) return;
                
                // 直接设置值，不做多余日志
                const nativeInputValueSetter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, 'value').set;
                
                element.focus();
                nativeInputValueSetter.call(element, value);
                
                // 立即触发事件
                element.dispatchEvent(new Event('input', {{ bubbles: true }})); 
                element.dispatchEvent(new Event('change', {{ bubbles: true }}));
            }}
            
            // 设置下拉框的值
            function setSelectValue(element, value) {{
                if (!element) return;
                console.log('[AutoFill] 设置下拉框:', element.id, '值:', value);
                element.value = value;
                element.dispatchEvent(new Event('change', {{ bubbles: true }}));
            }}
            
            // 等待元素出现 - 快速版本
            function waitForElement(selector, callback, timeout = 10000) {{
                const startTime = Date.now();
                const checkElement = () => {{
                    // 尝试多种方式查找元素
                    let element = document.querySelector(selector);
                    
                    // 如果通过ID找不到，尝试通过name属性
                    if (!element && selector.startsWith('#')) {{
                        const name = selector.substring(1);
                        element = document.querySelector(`input[name="${{name}}"]`) || 
                                 document.querySelector(`select[name="${{name}}"]`);
                    }}
                    
                    // 尝试通过placeholder查找
                    if (!element) {{
                        if (selector === '#cardNumber') {{
                            element = document.querySelector('input[placeholder*="1234"]');
                        }} else if (selector === '#cardExpiry') {{
                            element = document.querySelector('input[placeholder*="/"]') || 
                                     document.querySelector('input[placeholder*="月"]');
                        }} else if (selector === '#cardCvc') {{
                            element = document.querySelector('input[placeholder*="CVC"]') || 
                                     document.querySelector('input[placeholder*="CVV"]');
                        }} else if (selector === '#billingName') {{
                            element = document.querySelector('input[placeholder*="全名"]') || 
                                     document.querySelector('input[placeholder*="姓名"]');
                        }}
                    }}
                    
                    if (element) {{
                        console.log('[AutoFill] ✓ 找到元素:', selector);
                        callback(element);
                    }} else if (Date.now() - startTime > timeout) {{
                        console.error('[AutoFill] ✗ 元素未找到（超时）:', selector);
                    }} else {{
                        setTimeout(checkElement, 50); // 更频繁地检查
                    }}
                }};
                checkElement();
            }}
            
            // 填充表单的主函数
            function fillForm() {{
                console.log('[AutoFill] 准备填写表单...');
                console.log('[AutoFill] 当前URL:', window.location.href);
                console.log('[AutoFill] 页面语言:', document.documentElement.lang);
                
                // 并行填写所有卡片信息字段
                waitForElement('#cardNumber', (element) => {{
                    simulateTyping(element, '{}');
                }});
                
                waitForElement('#cardExpiry', (element) => {{
                    simulateTyping(element, '{}');
                }});
                
                waitForElement('#cardCvc', (element) => {{
                    simulateTyping(element, '{}');
                }});
                
                waitForElement('#billingName', (element) => {{
                    simulateTyping(element, '{}');
                }});
                
                // 并行处理地址信息
                // 先选择国家（这个必须先做）
                waitForElement('#billingCountry', (element) => {{
                    element.value = '{}';  // 国家代码
                    element.dispatchEvent(new Event('change', {{ bubbles: true }}));
                    console.log('✓ 已选择国家：{}');
                    
                    // 国家选择后立即开始填写其他地址信息
                    setTimeout(() => {{
                        // 填写邮编
                        waitForElement('#billingPostalCode', (element) => {{
                            simulateTyping(element, '{}');
                            console.log('✓ 已填写邮编');
                        }});
                        
                        // 填写省份（中国的省份选项需要等待加载）
                        waitForElement('#billingAdministrativeArea', (element) => {{
                            // 尝试找到匹配的省份选项
                            const options = element.querySelectorAll('option');
                            let stateSet = false;
                            const targetState = '{}';
                            for (const option of options) {{
                                if (option.value === targetState || option.value.includes(targetState) || option.text.includes(targetState)) {{
                                    element.value = option.value;
                                    element.dispatchEvent(new Event('change', {{ bubbles: true }}));
                                    console.log('✓ 已选择省份:', option.value);
                                    stateSet = true;
                                    break;
                                }}
                            }}
                            if (!stateSet) {{
                                console.warn('未找到匹配的省份选项，尝试直接设置');
                                element.value = targetState;
                                element.dispatchEvent(new Event('change', {{ bubbles: true }}));
                            }}
                        }});
                        
                        // 填写城市
                        waitForElement('#billingLocality', (element) => {{
                            simulateTyping(element, '{}');
                            console.log('✓ 已填写城市');
                        }});
                        
                        // 填写地区（中国地址特有）
                        waitForElement('#billingDependentLocality', (element) => {{
                            const district = '{}';
                            if (district && district.trim() !== '') {{
                                simulateTyping(element, district);
                                console.log('✓ 已填写地区:', district);
                            }}
                        }});
                        
                        // 填写地址第一行
                        waitForElement('#billingAddressLine1', (element) => {{
                            simulateTyping(element, '{}');
                            console.log('✓ 已填写地址第1行');
                        }});
                        
                        // 填写地址第二行
                        waitForElement('#billingAddressLine2', (element) => {{
                            const line2 = '{}';
                            console.log('[AutoFill] 准备填写地址第2行:', line2);
                            if (line2 && line2.trim() !== '') {{
                                simulateTyping(element, line2);
                                console.log('✓ 已填写地址第2行:', line2);
                            }} else {{
                                console.log('⚠️ 地址第2行为空，跳过填写');
                            }}
                        }});
                        
                        console.log('[AutoFill] 🎉 表单填写完成！');
                    }}, 500); // 等待省份选项加载
                }});
            }}
            
            // 卡段范围配置（用于重试时生成新卡号）
            const cardBinRange = '{}';  // 格式: "626200-626300" 或空
            const defaultCardBin = '{}';  // 默认卡头
            const maxRetries = {};  // 最大重试次数（从设置获取）
            const testModeEnabled = {};  // 测试模式（顺序遍历BIN）
            const useLocalBinPool = {};  // 使用本地BIN池（随机重试）
            const localBinPool = {};  // 本地BIN池内容
            let retryCount = 0;
            let currentSequentialBin = '{}';  // 当前顺序BIN（测试模式用）
            
            // Luhn算法生成校验位
            function calculateLuhnCheckDigit(partialNumber) {{
                let sum = 0;
                let isEven = true;
                for (let i = partialNumber.length - 1; i >= 0; i--) {{
                    let digit = parseInt(partialNumber[i], 10);
                    if (isEven) {{
                        digit *= 2;
                        if (digit > 9) digit -= 9;
                    }}
                    sum += digit;
                    isEven = !isEven;
                }}
                return (10 - (sum % 10)) % 10;
            }}
            
            // 顺序获取下一个BIN（测试模式用）
            function getNextSequentialBin() {{
                if (cardBinRange && cardBinRange.includes('-')) {{
                    const [startStr, endStr] = cardBinRange.split('-');
                    const start = parseInt(startStr.trim(), 10);
                    const end = parseInt(endStr.trim(), 10);
                    const current = parseInt(currentSequentialBin, 10);
                    
                    if (!isNaN(start) && !isNaN(end) && !isNaN(current) && end >= start) {{
                        let nextBin = current + 1;
                        if (nextBin > end) {{
                            nextBin = start;  // 循环回到起点
                        }}
                        currentSequentialBin = nextBin.toString().padStart(startStr.trim().length, '0');
                        console.log('[AutoFill] 测试模式 - 顺序获取下一个BIN:', currentSequentialBin);
                        return currentSequentialBin;
                    }}
                }}
                return defaultCardBin;
            }}
            
            // 从卡段范围随机选择BIN
            function getRandomBin() {{
                if (cardBinRange && cardBinRange.includes('-')) {{
                    const [startStr, endStr] = cardBinRange.split('-');
                    const start = parseInt(startStr.trim(), 10);
                    const end = parseInt(endStr.trim(), 10);
                    if (!isNaN(start) && !isNaN(end) && end >= start) {{
                        const randomBin = Math.floor(Math.random() * (end - start + 1)) + start;
                        return randomBin.toString();
                    }}
                }}
                return defaultCardBin;
            }}
            
            // 从本地BIN池随机选择
            function getRandomBinFromPool() {{
                if (localBinPool && localBinPool.length > 0) {{
                    const randomIndex = Math.floor(Math.random() * localBinPool.length);
                    const bin = localBinPool[randomIndex];
                    console.log('[AutoFill] 从BIN池随机抽取:', bin);
                    return bin;
                }}
                console.log('[AutoFill] BIN池为空，使用默认BIN');
                return defaultCardBin;
            }}
            
            // 获取BIN（根据模式选择）
            function getBin() {{
                // 测试模式：顺序遍历
                if (testModeEnabled) {{
                    return getNextSequentialBin();
                }}
                // 本地BIN池模式：从池中随机抽取
                if (useLocalBinPool) {{
                    return getRandomBinFromPool();
                }}
                return getRandomBin();
            }}
            
            // 生成卡号（使用指定BIN）
            function generateCardNumberWithBin(bin) {{
                const binLength = bin.length;
                const randomDigits = 16 - binLength - 1;
                let cardNumber = bin;
                for (let i = 0; i < randomDigits; i++) {{
                    cardNumber += Math.floor(Math.random() * 10);
                }}
                cardNumber += calculateLuhnCheckDigit(cardNumber);
                return cardNumber;
            }}
            
            // 生成随机卡号
            function generateCardNumber() {{
                const bin = getBin();
                return generateCardNumberWithBin(bin);
            }}
            
            // 生成随机有效期
            function generateExpiryDate() {{
                const currentYear = new Date().getFullYear();
                const year = currentYear + Math.floor(Math.random() * 5) + 1;
                const month = Math.floor(Math.random() * 12) + 1;
                return `${{month.toString().padStart(2, '0')}}/${{(year % 100).toString().padStart(2, '0')}}`;
            }}
            
            // 生成随机CVV
            function generateCvv() {{
                return Math.floor(Math.random() * 900 + 100).toString();
            }}
            
            // 清空并重新填写卡信息
            function clearAndRefillCard() {{
                retryCount++;
                console.log(`[AutoFill] 🔄 重试第 ${{retryCount}} 次...`);
                
                const newCardNumber = generateCardNumber();
                const newExpiry = generateExpiryDate();
                const newCvv = generateCvv();
                
                console.log('[AutoFill] 新卡号:', newCardNumber);
                
                // 清空并重新填写
                const cardNumberEl = document.querySelector('#cardNumber') || document.querySelector('input[name="cardNumber"]');
                const expiryEl = document.querySelector('#cardExpiry') || document.querySelector('input[name="cardExpiry"]');
                const cvvEl = document.querySelector('#cardCvc') || document.querySelector('input[name="cardCvc"]');
                
                if (cardNumberEl) {{
                    cardNumberEl.value = '';
                    cardNumberEl.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    setTimeout(() => simulateTyping(cardNumberEl, newCardNumber), 100);
                }}
                if (expiryEl) {{
                    expiryEl.value = '';
                    expiryEl.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    setTimeout(() => simulateTyping(expiryEl, newExpiry), 200);
                }}
                if (cvvEl) {{
                    cvvEl.value = '';
                    cvvEl.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    setTimeout(() => simulateTyping(cvvEl, newCvv), 300);
                }}
                
                // 重新提交
                setTimeout(() => {{
                    const submitBtn = document.querySelector('button[type="submit"]');
                    if (submitBtn && !submitBtn.disabled) {{
                        console.log('[AutoFill] 重新提交表单...');
                        submitBtn.click();
                    }}
                }}, 1000);
            }}
            
            // 监控提交结果（使用轮询方式，更可靠）
            let monitorInterval = null;
            
            function monitorSubmitResult() {{
                console.log('[AutoFill] 开始轮询监控提交结果...');
                
                monitorInterval = setInterval(() => {{
                    const submitBtn = document.querySelector('button[type="submit"]');
                    if (!submitBtn) return;
                    
                    // 检查是否绑卡成功（检查按钮类名或勾选图标）
                    const hasSuccessClass = submitBtn.classList.contains('SubmitButton--success');
                    const hasCheckmark = submitBtn.querySelector('.SubmitButton-CheckmarkIcon--current');
                    
                    if (hasSuccessClass || hasCheckmark) {{
                        console.log('[AutoFill] ✅ 绑卡成功！');
                        clearInterval(monitorInterval);
                        
                        // 获取当前卡号的BIN（前6位）
                        let currentBin = '';
                        const cardInput = document.querySelector('#cardNumber') || document.querySelector('input[name="cardNumber"]');
                        if (cardInput && cardInput.value) {{
                            currentBin = cardInput.value.replace(/\D/g, '').substring(0, 6);
                        }}
                        
                        // 通过修改 URL hash 发送成功信号（包含当前BIN）
                        window.location.hash = '#___PAYMENT_SUCCESS___BIN_' + currentBin;
                        document.title = '___PAYMENT_SUCCESS___';
                        console.log('[AutoFill] 已发送成功信号，当前BIN:', currentBin);
                        return;
                    }}
                    
                    // 检查是否有错误提示（卡被拒绝、验证失败等）
                    // 优先检查特定的错误容器
                    let errorEl = document.querySelector('.ConfirmPaymentButton-Error');
                    if (!errorEl) {{
                        errorEl = document.querySelector(
                            '.FieldError:not(:empty), ' +
                            '.Error:not(:empty), ' +
                            '.Notice-message:not(:empty), ' +
                            '.Notice--red:not(:empty), ' +
                            '[class*="error"]:not(:empty), ' +
                            '[class*="Error"]:not(:empty), ' +
                            '[class*="decline"]:not(:empty)'
                        );
                    }}
                    const errorText = errorEl ? errorEl.textContent.trim() : '';
                    
                    // 调试日志
                    if (errorEl) {{
                        console.log('[AutoFill] 检测到错误元素:', errorEl.className);
                    }}
                    
                    // 当按钮可点击且有错误信息时检查是否需要重试
                    if (errorText && !submitBtn.disabled) {{
                        // 检查是否是新的错误（通过时间戳判断，避免重复触发）
                        const now = Date.now();
                        if (!window.__lastRetryTime || now - window.__lastRetryTime > 3000) {{
                            console.log('[AutoFill] ❌ 绑卡失败，错误信息:', errorText);
                            
                            if (retryCount < maxRetries) {{
                                window.__lastRetryTime = now;
                                console.log(`[AutoFill] 准备重试... (${{retryCount + 1}}/${{maxRetries}})`);
                                setTimeout(clearAndRefillCard, 1500);
                            }} else {{
                                console.log('[AutoFill] ⚠️ 已达到最大重试次数，发送失败信号');
                                clearInterval(monitorInterval);
                                // 发送失败信号，包含当前BIN让Rust端保存进度
                                window.location.hash = '#___PAYMENT_FAILED___BIN_' + currentSequentialBin;
                            }}
                        }}
                    }}
                }}, 500); // 每500ms检测一次
                
                // 300秒后停止监控（超时保护）
                setTimeout(() => {{
                    if (monitorInterval) {{
                        console.log('[AutoFill] 监控超时(300s)，停止轮询');
                        clearInterval(monitorInterval);
                    }}
                }}, 300000);
            }}
            
            // 快速启动填写
            const testElement = document.querySelector('input') || document.querySelector('select');
            if (testElement) {{
                console.log('[AutoFill] 页面已加载，立即开始填写');
                setTimeout(fillForm, 500);
                setTimeout(monitorSubmitResult, 1000);
            }} else {{
                console.log('[AutoFill] 等待DOM加载...');
                if (document.readyState === 'complete' || document.readyState === 'interactive') {{
                    setTimeout(fillForm, 1000);
                    setTimeout(monitorSubmitResult, 1500);
                }} else {{
                    document.addEventListener('DOMContentLoaded', () => {{
                        console.log('[AutoFill] DOM已加载');
                        setTimeout(fillForm, 500);
                        setTimeout(monitorSubmitResult, 1000);
                    }});
                }}
            }}
        }})();
    "#,
        card_info.card_number.replace(" ", ""),  // 移除空格
        card_info.expiry_date,
        card_info.cvv,
        card_info.cardholder_name,
        card_info.billing_address.country,  // 国家代码
        if card_info.billing_address.country == "CN" { "中国" } else { "美国" },  // 国家名称
        card_info.billing_address.postal_code,
        card_info.billing_address.state,
        card_info.billing_address.city,
        card_info.billing_address.district,  // 地区
        card_info.billing_address.street_address,
        card_info.billing_address.street_address_line2,
        card_bin_range.clone().unwrap_or_default(),  // 卡段范围
        custom_bin.clone(),  // 默认卡头
        max_retries,  // 最大重试次数
        test_mode_enabled,  // 测试模式
        use_local_bin_pool,  // 使用本地BIN池
        serde_json::to_string(&local_bin_pool).unwrap_or_else(|_| "[]".to_string()),  // 本地BIN池
        custom_bin.clone()  // 当前顺序BIN（初始值）
    );
    
    // 执行JavaScript代码
    window.eval(&js_code).map_err(|e| {
        eprintln!("执行JavaScript失败: {}", e);
        e.to_string()
    })?;
    
    println!("[AutoFill] JavaScript已注入到窗口: {}", window_label);
    
    // 提取当前卡号的 BIN（前6位，需要先移除空格）
    let current_bin = card_info.card_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(6)
        .collect::<String>();
    
    // 启动后台任务监控绑卡结果
    let window_for_monitor = window.clone();
    let window_label_for_log = window_label.clone();
    let app_for_monitor = app.clone();
    let data_store_for_monitor = data_store.clone();
    let test_mode = test_mode_enabled;
    let bin_to_save = current_bin.clone();
    tokio::spawn(async move {
        println!("[AutoFill] 开始监控绑卡结果... (当前BIN: {})", bin_to_save);
        let mut check_count = 0;
        let max_checks = 600; // 最多检测300秒 (600 * 500ms)
        
        // 等待表单填写完成
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            check_count += 1;
            
            if check_count > max_checks {
                println!("[AutoFill] 监控超时，停止检测");
                break;
            }
            
            // 检查窗口是否还存在
            if !window_for_monitor.is_visible().unwrap_or(false) {
                println!("[AutoFill] 窗口已关闭，停止监控");
                break;
            }
            
            // 直接执行 JS 检查按钮状态并关闭窗口
            let check_and_close_js = r#"
                (function() {
                    var btn = document.querySelector('button[type="submit"]');
                    if (btn && (btn.classList.contains('SubmitButton--success') || 
                                btn.querySelector('.SubmitButton-CheckmarkIcon--current'))) {
                        console.log('[Rust监控] 检测到成功状态！');
                        // 标记成功
                        window.__PAYMENT_SUCCESS__ = true;
                        return true;
                    }
                    return false;
                })();
            "#;
            
            if let Err(e) = window_for_monitor.eval(check_and_close_js) {
                println!("[AutoFill] 窗口已关闭或执行失败: {}", e);
                break;
            }
            
            // 检查 URL hash
            if let Ok(url) = window_for_monitor.url() {
                let url_str = url.to_string();
                
                // 检测成功信号
                if url_str.contains("___PAYMENT_SUCCESS___") {
                    println!("[AutoFill] ✅ 从 URL 检测到成功信号！关闭窗口: {}", window_label_for_log);
                    
                    // 如果开启了测试模式，从URL hash中解析BIN并保存
                    if test_mode {
                        // 从 URL 中提取 BIN (格式: #___PAYMENT_SUCCESS___BIN_628296)
                        let current_bin = if let Some(bin_start) = url_str.find("BIN_") {
                            let bin_part = &url_str[bin_start + 4..];
                            // 取前6位数字
                            bin_part.chars().take(6).collect::<String>()
                        } else {
                            bin_to_save.clone()
                        };
                        
                        if current_bin.len() == 6 && current_bin.chars().all(|c| c.is_ascii_digit()) {
                            // 保存到成功BIN池
                            if let Err(e) = add_success_bin(app_for_monitor.clone(), current_bin.clone()).await {
                                println!("[AutoFill] 保存成功BIN失败: {}", e);
                            } else {
                                println!("[AutoFill] 📝 已保存成功BIN: {}", current_bin);
                            }
                            
                            // 更新进度（保存实际成功的BIN，下次从这个BIN+1开始）
                            if let Ok(mut settings) = data_store_for_monitor.get_settings().await {
                                settings.test_mode_last_bin = Some(current_bin.clone());
                                if let Err(e) = data_store_for_monitor.update_settings(settings).await {
                                    println!("[AutoFill] 更新进度失败: {}", e);
                                } else {
                                    println!("[AutoFill] 📍 更新进度到: {}", current_bin);
                                }
                            }
                        }
                    }
                    
                    tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
                    let _ = window_for_monitor.close();
                    break;
                }
                
                // 检测失败信号（重试次数用完）
                if url_str.contains("___PAYMENT_FAILED___") {
                    // 从 URL 中提取最后尝试的 BIN (格式: #___PAYMENT_FAILED___BIN_628296)
                    if let Some(bin_start) = url_str.find("BIN_") {
                        let bin_part = &url_str[bin_start + 4..];
                        let last_bin: String = bin_part.chars().take(6).collect();
                        
                        if last_bin.len() == 6 && last_bin.chars().all(|c| c.is_ascii_digit()) {
                            println!("[AutoFill] ❌ 重试次数已用完，保存进度BIN: {}", last_bin);
                            // 保存进度
                            if let Ok(mut settings) = data_store_for_monitor.get_settings().await {
                                settings.test_mode_last_bin = Some(last_bin.clone());
                                if let Err(e) = data_store_for_monitor.update_settings(settings).await {
                                    println!("[AutoFill] 保存BIN进度失败: {}", e);
                                }
                            }
                        }
                    }
                    
                    println!("[AutoFill] 关闭窗口: {}", window_label_for_log);
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    let _ = window_for_monitor.close();
                    break;
                }
            }
            
            // 每 10 次检查输出一次日志
            if check_count % 10 == 0 {
                println!("[AutoFill] 检测中... ({}/{})", check_count, max_checks);
            }
        }
    });
    
    Ok(())
}

#[command]
pub async fn validate_card_number(card_number: String) -> bool {
    CardGenerator::validate_card_number(&card_number)
}

/// 关闭支付窗口（绑卡成功后由前端调用）
#[command]
pub async fn close_payment_window(app: AppHandle) -> Result<(), String> {
    println!("[Payment] 收到关闭窗口请求");
    
    // 查找并关闭所有以 payment_ 开头的窗口
    for window in app.webview_windows().values() {
        let label = window.label();
        if label.starts_with("payment_") {
            println!("[Payment] 关闭窗口: {}", label);
            let _ = window.close();
        }
    }
    
    Ok(())
}

/// 获取试用绑卡链接并可选地在内置浏览器中打开（增强版）
#[command]
pub async fn get_trial_payment_link_enhanced(
    app: AppHandle,
    data_store: State<'_, Arc<DataStore>>,
    account_name: String,
    token: String,
    auto_open: bool,
    teams_tier: i32,
    payment_period: i32,
    team_name: Option<String>,
    seat_count: Option<i32>,
    turnstile_token: Option<String>,
) -> Result<serde_json::Value, String> {
    // 获取WindsurfService实例
    let service = crate::services::windsurf_service::WindsurfService::new();
    
    // 调用subscribe_to_plan方法获取支付链接
    let result = service.subscribe_to_plan(
        &token, 
        teams_tier,
        payment_period,
        team_name.as_deref(),
        seat_count,
        turnstile_token.as_deref()
    )
        .await
        .map_err(|e| e.to_string())?;
    
    // 检查是否成功
    let success = result.get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
        
    if !success {
        return Ok(result);
    }
    
    // 如果成功获取链接
    if let Some(stripe_url) = result.get("stripe_url").and_then(|v| v.as_str()) {
        if !stripe_url.is_empty() && auto_open {
            // 打开支付窗口（无痕模式）
            let window_label = open_payment_window(app.clone(), stripe_url.to_string(), account_name.clone())
                .await
                .map_err(|e| e.to_string())?;
            
            // 获取设置中的自定义卡头和卡段范围并生成虚拟卡信息
            let settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
            let custom_bin = settings.custom_card_bin;
            let bin_range = settings.custom_card_bin_range;
            let virtual_card = CardGenerator::generate_card_with_bin_or_range(&custom_bin, bin_range.as_deref());
            
            println!("已在无痕模式下打开支付窗口: {}", window_label);
            
            // 返回包含虚拟卡信息和窗口标签的结果
            return Ok(json!({
                "success": true,
                "stripe_url": stripe_url,
                "virtual_card": virtual_card,
                "window_opened": true,
                "window_label": window_label,
                "incognito_mode": true,  // 标记使用了无痕模式
                "teams_tier": teams_tier,
                "payment_period": payment_period,
                "account_name": account_name,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }));
        }
    }
    
    // 返回原始结果
    Ok(result)
}

/// 在系统默认浏览器中打开链接
#[command]
pub async fn open_external_link(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", "", &url])  // 添加空字符串作为窗口标题参数
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

/// 在浏览器无痕模式中打开链接
#[command]
pub async fn open_external_link_incognito(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // 尝试使用 Chrome 无痕模式
        let chrome_result = Command::new("cmd")
            .args(&["/C", "start", "chrome", "--incognito", &url])
            .spawn();
        
        if chrome_result.is_ok() {
            return Ok(());
        }
        
        // 如果 Chrome 失败，尝试 Edge 无痕模式
        let edge_result = Command::new("cmd")
            .args(&["/C", "start", "msedge", "-inprivate", &url])
            .spawn();
        
        if edge_result.is_ok() {
            return Ok(());
        }
        
        // 如果都失败，回退到默认浏览器
        Command::new("cmd")
            .args(&["/C", "start", "", &url])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS 上尝试使用 Chrome 无痕模式
        let chrome_result = Command::new("open")
            .args(&["-na", "Google Chrome", "--args", "--incognito", &url])
            .spawn();
        
        if chrome_result.is_ok() {
            return Ok(());
        }
        
        // 回退到默认浏览器
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux 上尝试使用 Chrome 无痕模式
        let chrome_result = Command::new("google-chrome")
            .args(&["--incognito", &url])
            .spawn();
        
        if chrome_result.is_ok() {
            return Ok(());
        }
        
        // 尝试 chromium
        let chromium_result = Command::new("chromium")
            .args(&["--incognito", &url])
            .spawn();
        
        if chromium_result.is_ok() {
            return Ok(());
        }
        
        // 回退到默认浏览器
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

/// 自动填写支付表单
#[command]
pub async fn auto_fill_payment_form(
    app: AppHandle,
    data_store: State<'_, Arc<DataStore>>,
    window_label: String,
    virtual_card: Option<VirtualCard>,
) -> Result<(), String> {
    // 获取设置中的自定义卡头和卡段范围
    let settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
    let mut custom_bin = settings.custom_card_bin.clone();
    let bin_range = settings.custom_card_bin_range.clone();
    let mut use_test_mode_sequential = false;  // 标记是否使用测试模式顺序逻辑
    let mut use_local_bin_pool_mode = false;  // 标记是否使用本地BIN池模式
    
    // 如果开启了使用本地BIN池，尝试从池中获取BIN（独立运行，不需要测试模式）
    if settings.use_local_success_bins {
        if let Ok(Some(success_bin)) = get_random_success_bin(app.clone()).await {
            println!("[AutoFill] 从成功BIN池随机抽取: {}", success_bin);
            custom_bin = success_bin;
            use_local_bin_pool_mode = true;
        } else {
            println!("[AutoFill] BIN池为空，使用默认设置");
        }
    }
    // 如果开启了测试模式，顺序遍历BIN范围
    else if settings.test_mode_enabled && bin_range.is_some() {
        use_test_mode_sequential = true;
        // 重新获取最新设置，确保获取到重置后的状态
        let fresh_settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
        let last_bin = fresh_settings.test_mode_last_bin.as_deref();
        println!("[AutoFill] 测试模式 - 从设置读取的上次BIN: {:?}", last_bin);
        
        let (next_bin, is_end) = CardGenerator::get_next_bin_from_range(
            &custom_bin,
            bin_range.as_deref(),
            last_bin,
        );
        println!("[AutoFill] 测试模式 - 顺序获取BIN: {} (上次: {:?}, 是否到末尾: {})", 
            next_bin, last_bin, is_end);
        custom_bin = next_bin.clone();
        
        // 保存当前BIN进度
        let mut new_settings = fresh_settings.clone();
        new_settings.test_mode_last_bin = Some(next_bin);
        if let Err(e) = data_store.update_settings(new_settings).await {
            println!("[AutoFill] 保存BIN进度失败: {}", e);
        }
    }
    
    // 如果没有提供虚拟卡信息，则生成一个新的
    let card = if let Some(card) = virtual_card {
        card
    } else if use_test_mode_sequential {
        // 测试模式：使用指定的顺序 BIN（不使用范围随机）
        println!("[AutoFill] 测试模式生成卡号，使用BIN: {}", custom_bin);
        let c = CardGenerator::generate_card_with_bin(&custom_bin);
        println!("[AutoFill] 生成的卡号: {}", c.card_number);
        c
    } else if use_local_bin_pool_mode {
        // 本地BIN池模式：使用池中随机抽取的BIN
        println!("[AutoFill] BIN池模式生成卡号，使用BIN: {}", custom_bin);
        let c = CardGenerator::generate_card_with_bin(&custom_bin);
        println!("[AutoFill] 生成的卡号: {}", c.card_number);
        c
    } else {
        CardGenerator::generate_card_with_bin_or_range(&custom_bin, bin_range.as_deref())
    };
    
    // 获取本地BIN池（用于JS重试时随机选择）
    let local_bin_pool = if settings.use_local_success_bins {
        get_success_bins(app.clone()).await.unwrap_or_default()
    } else {
        vec![]
    };
    
    // 注入卡信息（直接调用内部实现）
    // 测试模式下保留原始 bin_range，让 JS 可以计算下一个顺序 BIN
    let original_bin_range = settings.custom_card_bin_range.clone();
    inject_card_info_internal(
        app,
        data_store.inner().clone(),
        window_label,
        card,
        if settings.test_mode_enabled { original_bin_range } else { bin_range },
        custom_bin,
        settings.card_bind_retry_times,
        settings.test_mode_enabled,
        settings.use_local_success_bins,
        local_bin_pool,
    ).await?;
    
    Ok(())
}

/// 注入自动提交脚本
#[command]
pub async fn inject_auto_submit_script(
    app: AppHandle,
    window_label: String,
) -> Result<(), String> {
    // 获取窗口
    let window = app.get_webview_window(&window_label)
        .ok_or("Window not found".to_string())?;
    
    // 构建自动提交的JavaScript代码
    let js_code = r#"
        (function() {
            console.log('[AutoSubmit] 自动提交脚本已注入');
            
            // 等待提交按钮变为可点击状态
            function waitForSubmitButtonReady(timeout = 30000) {
                const startTime = Date.now();
                
                return new Promise((resolve) => {
                    const checkButton = () => {
                        // 查找提交按钮
                        const submitButton = document.querySelector('button[type="submit"]');
                        
                        if (submitButton) {
                            // 检查按钮是否包含 complete 类名
                            const isComplete = submitButton.classList.contains('SubmitButton--complete');
                            // 检查按钮文字
                            const buttonText = submitButton.querySelector('.SubmitButton-Text--current')?.textContent;
                            const isReady = buttonText?.includes('开始试用') || buttonText?.includes('Start trial');
                            
                            console.log('[AutoSubmit] 按钮状态:', {
                                isComplete,
                                buttonText,
                                disabled: submitButton.disabled
                            });
                            
                            if (isComplete && !submitButton.disabled) {
                                console.log('[AutoSubmit] ✅ 提交按钮已就绪');
                                resolve(submitButton);
                                return;
                            } else if (!isComplete) {
                                console.log('[AutoSubmit] ⏳ 等待按钮变为complete状态...');
                            }
                        }
                        
                        // 检查是否超时
                        if (Date.now() - startTime > timeout) {
                            console.error('[AutoSubmit] ❌ 等待提交按钮超时');
                            resolve(null);
                        } else {
                            setTimeout(checkButton, 1000);
                        }
                    };
                    
                    checkButton();
                });
            }
            
            // 自动提交流程
            async function autoSubmit() {
                console.log('[AutoSubmit] 等待5秒后开始自动提交流程...');
                await new Promise(resolve => setTimeout(resolve, 5000));
                
                console.log('[AutoSubmit] 🔍 正在等待提交按钮变为可点击状态...');
                const submitButton = await waitForSubmitButtonReady();
                
                if (submitButton) {
                    // 滚动到按钮位置
                    submitButton.scrollIntoView({ behavior: 'smooth', block: 'center' });
                    await new Promise(resolve => setTimeout(resolve, 500));
                    
                    // 点击提交按钮
                    console.log('[AutoSubmit] 🖱️ 点击提交按钮');
                    submitButton.click();
                    
                    // 1秒后再次点击以确保提交
                    setTimeout(() => {
                        if (submitButton && !submitButton.disabled) {
                            submitButton.click();
                            console.log('[AutoSubmit] ✅ 再次确认点击提交按钮');
                        }
                    }, 1000);
                } else {
                    console.error('[AutoSubmit] ❌ 未找到可用的提交按钮');
                }
            }
            
            // 启动自动提交
            autoSubmit();
        })();
    "#.to_string();
    
    // 执行JavaScript代码
    window.eval(&js_code).map_err(|e| {
        eprintln!("执行自动提交脚本失败: {}", e);
        e.to_string()
    })?;
    
    println!("[AutoSubmit] 自动提交脚本已注入到窗口: {}", window_label);
    
    Ok(())
}

// ========== 成功BIN池管理 ==========

use std::path::PathBuf;

/// 获取成功BIN池文件路径
fn get_success_bins_file_path(app: &AppHandle) -> PathBuf {
    let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    app_data_dir.join("success_bins.json")
}

/// 获取成功BIN列表
#[command]
pub async fn get_success_bins(app: AppHandle) -> Result<Vec<String>, String> {
    let file_path = get_success_bins_file_path(&app);
    if !file_path.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let bins: Vec<String> = serde_json::from_str(&content).unwrap_or_default();
    Ok(bins)
}

/// 添加成功BIN到池中
#[command]
pub async fn add_success_bin(app: AppHandle, bin: String) -> Result<(), String> {
    let file_path = get_success_bins_file_path(&app);
    
    // 读取现有列表
    let mut bins: Vec<String> = if file_path.exists() {
        let content = fs::read_to_string(&file_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    // 检查是否已存在
    if !bins.contains(&bin) {
        bins.push(bin.clone());
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        
        // 保存到文件
        let content = serde_json::to_string_pretty(&bins).map_err(|e| e.to_string())?;
        fs::write(&file_path, content).map_err(|e| e.to_string())?;
        
        println!("[BIN池] 已添加成功BIN: {}", bin);
    }
    
    Ok(())
}

/// 清空成功BIN池
#[command]
pub async fn clear_success_bins(app: AppHandle) -> Result<(), String> {
    let file_path = get_success_bins_file_path(&app);
    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| e.to_string())?;
    }
    println!("[BIN池] 已清空成功BIN池");
    Ok(())
}

/// 从成功BIN池中随机获取一个BIN
#[command]
pub async fn get_random_success_bin(app: AppHandle) -> Result<Option<String>, String> {
    let bins = get_success_bins(app).await?;
    if bins.is_empty() {
        return Ok(None);
    }
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..bins.len());
    Ok(Some(bins[index].clone()))
}

/// 重置测试模式的BIN遍历进度
#[command]
pub async fn reset_test_mode_progress(
    data_store: State<'_, Arc<DataStore>>,
) -> Result<(), String> {
    println!("[TestMode] 开始重置进度...");
    let mut settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
    println!("[TestMode] 重置前 last_bin: {:?}", settings.test_mode_last_bin);
    settings.test_mode_last_bin = None;
    data_store.update_settings(settings.clone()).await.map_err(|e| e.to_string())?;
    
    // 验证是否保存成功
    let verify = data_store.get_settings().await.map_err(|e| e.to_string())?;
    println!("[TestMode] 重置后验证 last_bin: {:?}", verify.test_mode_last_bin);
    
    if verify.test_mode_last_bin.is_some() {
        return Err("重置失败：进度未能清除".to_string());
    }
    
    println!("[TestMode] ✓ 已重置BIN遍历进度");
    Ok(())
}

/// 获取测试模式当前进度信息
#[command]
pub async fn get_test_mode_progress(
    data_store: State<'_, Arc<DataStore>>,
) -> Result<Option<String>, String> {
    let settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
    Ok(settings.test_mode_last_bin)
}
