use rand::Rng;
use serde::{Deserialize, Serialize};
use chrono::{Utc, Datelike};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VirtualCard {
    pub card_number: String,
    pub expiry_date: String,
    pub cvv: String,
    pub cardholder_name: String,
    pub billing_address: BillingAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BillingAddress {
    pub street_address: String,
    pub street_address_line2: String,  // 地址第二行
    pub city: String,
    pub district: String,  // 地区（中国地址使用）
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

pub struct CardGenerator;

impl CardGenerator {
    /// 生成完整的虚拟卡信息（使用默认BIN）
    #[allow(dead_code)]
    pub fn generate_card() -> VirtualCard {
        Self::generate_card_with_bin("626202")
    }
    
    /// 生成完整的虚拟卡信息（使用指定BIN）
    pub fn generate_card_with_bin(bin: &str) -> VirtualCard {
        VirtualCard {
            card_number: Self::generate_card_number_with_bin(bin),
            expiry_date: Self::generate_expiry_date(),
            cvv: Self::generate_cvv(),
            cardholder_name: Self::generate_name(),
            billing_address: Self::generate_address(),
        }
    }
    
    /// 生成完整的虚拟卡信息（支持卡段范围，如 "626200-626300"）
    /// 如果提供了有效的卡段范围，则从范围内随机选择一个BIN
    /// 否则使用指定的单一BIN
    pub fn generate_card_with_bin_or_range(bin: &str, bin_range: Option<&str>) -> VirtualCard {
        let actual_bin = Self::get_bin_from_range(bin, bin_range);
        Self::generate_card_with_bin(&actual_bin)
    }
    
    /// 从卡段范围中顺序获取下一个BIN（测试模式用）
    /// 返回 (下一个BIN, 是否已到达范围末尾)
    pub fn get_next_bin_from_range(
        default_bin: &str,
        bin_range: Option<&str>,
        last_bin: Option<&str>,
    ) -> (String, bool) {
        if let Some(range) = bin_range {
            let range = range.trim();
            if range.is_empty() {
                return (default_bin.to_string(), true);
            }
            
            // 解析范围格式：起始-结束
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() != 2 {
                return (default_bin.to_string(), true);
            }
            
            let start_str = parts[0].trim();
            let end_str = parts[1].trim();
            
            // 验证两端长度相同且都是数字
            if start_str.len() != end_str.len() 
                || !start_str.chars().all(|c| c.is_ascii_digit())
                || !end_str.chars().all(|c| c.is_ascii_digit()) 
            {
                return (default_bin.to_string(), true);
            }
            
            let start: u64 = match start_str.parse() {
                Ok(n) => n,
                Err(_) => return (default_bin.to_string(), true),
            };
            let end: u64 = match end_str.parse() {
                Ok(n) => n,
                Err(_) => return (default_bin.to_string(), true),
            };
            
            if start > end {
                return (default_bin.to_string(), true);
            }
            
            // 确定下一个BIN
            let next_bin: u64 = if let Some(last) = last_bin {
                if let Ok(last_num) = last.parse::<u64>() {
                    if last_num >= start && last_num < end {
                        last_num + 1
                    } else if last_num >= end {
                        // 已到末尾，从头开始
                        start
                    } else {
                        start
                    }
                } else {
                    start
                }
            } else {
                start
            };
            
            let is_end = next_bin >= end;
            let bin_str = format!("{:0width$}", next_bin, width = start_str.len());
            (bin_str, is_end)
        } else {
            (default_bin.to_string(), true)
        }
    }
    
    /// 从卡段范围中随机选择一个BIN
    /// 格式：起始BIN-结束BIN，如 "626200-626300"
    /// 如果范围无效，则返回默认BIN
    pub fn get_bin_from_range(default_bin: &str, bin_range: Option<&str>) -> String {
        if let Some(range) = bin_range {
            let range = range.trim();
            if range.is_empty() {
                return default_bin.to_string();
            }
            
            // 解析范围格式：起始-结束
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() != 2 {
                return default_bin.to_string();
            }
            
            let start_str = parts[0].trim();
            let end_str = parts[1].trim();
            
            // 验证两端长度相同且都是数字
            if start_str.len() != end_str.len() 
                || !start_str.chars().all(|c| c.is_ascii_digit())
                || !end_str.chars().all(|c| c.is_ascii_digit()) 
            {
                return default_bin.to_string();
            }
            
            // 解析为数字
            let start: u64 = match start_str.parse() {
                Ok(n) => n,
                Err(_) => return default_bin.to_string(),
            };
            let end: u64 = match end_str.parse() {
                Ok(n) => n,
                Err(_) => return default_bin.to_string(),
            };
            
            // 验证范围有效
            if start > end {
                return default_bin.to_string();
            }
            
            // 从范围内随机选择一个BIN
            let mut rng = rand::thread_rng();
            let selected = rng.gen_range(start..=end);
            
            // 保持原始长度（前导零）
            format!("{:0width$}", selected, width = start_str.len())
        } else {
            default_bin.to_string()
        }
    }

    /// 使用Luhn算法生成有效的16位卡号（使用默认BIN）
    #[allow(dead_code)]
    pub fn generate_card_number() -> String {
        Self::generate_card_number_with_bin("626202")
    }
    
    /// 使用Luhn算法生成有效的卡号（使用指定BIN）
    pub fn generate_card_number_with_bin(bin: &str) -> String {
        let mut rng = rand::thread_rng();
        
        // 验证BIN长度（4-12位）
        let bin_len = bin.len();
        if bin_len < 4 || bin_len > 12 {
            // 如果BIN长度无效，使用默认BIN
            return Self::generate_card_number_with_bin("626202");
        }
        
        // 验证BIN只包含数字
        if !bin.chars().all(|c| c.is_ascii_digit()) {
            // 如果BIN包含非数字字符，使用默认BIN
            return Self::generate_card_number_with_bin("626202");
        }
        
        // 计算需要生成的随机数字位数
        // 常见的卡号长度是16位，但也有15位（如AmEx）和19位的
        // 这里默认生成16位卡号
        let total_length = 16;
        let random_digits_count = total_length - bin_len - 1; // -1 for check digit
        
        // 生成中间的随机数字
        let mut card_number = String::from(bin);
        for _ in 0..random_digits_count {
            card_number.push_str(&rng.gen_range(0..10).to_string());
        }
        
        // 计算并添加Luhn校验位
        let check_digit = Self::calculate_luhn_check_digit(&card_number);
        card_number.push_str(&check_digit.to_string());
        
        // 格式化卡号（每4位一个空格）
        Self::format_card_number(&card_number)
    }

    /// 计算Luhn校验位
    fn calculate_luhn_check_digit(card_number: &str) -> u8 {
        let digits: Vec<u8> = card_number
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();
        
        let mut sum = 0;
        let mut double = true;
        
        for &digit in digits.iter().rev() {
            if double {
                let mut doubled = digit * 2;
                if doubled > 9 {
                    doubled = doubled - 9;
                }
                sum += doubled;
            } else {
                sum += digit;
            }
            double = !double;
        }
        
        let remainder = sum % 10;
        if remainder == 0 {
            0
        } else {
            10 - remainder
        }
    }

    /// 格式化卡号为 XXXX XXXX XXXX XXXX
    fn format_card_number(card_number: &str) -> String {
        card_number
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// 生成未来1-5年内的有效期
    pub fn generate_expiry_date() -> String {
        let mut rng = rand::thread_rng();
        let current_year = Utc::now().year();
        
        // 生成未来1-5年
        let year = current_year + rng.gen_range(1..=5);
        // 生成1-12月份
        let month = rng.gen_range(1..=12);
        
        format!("{:02}/{:02}", month, year % 100)
    }

    /// 生成3位CVC码
    pub fn generate_cvv() -> String {
        let mut rng = rand::thread_rng();
        format!("{:03}", rng.gen_range(100..1000))
    }

    /// 生成随机英文姓名
    pub fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        
        let first_names = vec![
            "John", "James", "Michael", "David", "William",
            "Robert", "Joseph", "Thomas", "Charles", "Christopher",
            "Daniel", "Matthew", "Anthony", "Mark", "Donald",
            "Mary", "Patricia", "Jennifer", "Linda", "Elizabeth",
            "Barbara", "Susan", "Jessica", "Sarah", "Karen"
        ];
        
        let last_names = vec![
            "Smith", "Johnson", "Williams", "Brown", "Jones",
            "Garcia", "Miller", "Davis", "Rodriguez", "Martinez",
            "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson",
            "Thomas", "Taylor", "Moore", "Jackson", "Martin",
            "Lee", "Perez", "Thompson", "White", "Harris"
        ];
        
        let first_name = first_names[rng.gen_range(0..first_names.len())];
        let last_name = last_names[rng.gen_range(0..last_names.len())];
        
        format!("{} {}", first_name, last_name)
    }

    /// 生成随机中国地址
    pub fn generate_address() -> BillingAddress {
        let mut rng = rand::thread_rng();
        
        // 中国省市区数据：(省份, 城市, 区县, 邮编前缀)
        let china_addresses = vec![
            // 广东省
            ("广东省", "深圳市", vec!["南山区", "福田区", "罗湖区", "宝安区", "龙岗区", "龙华区"], "518"),
            ("广东省", "广州市", vec!["天河区", "越秀区", "海珠区", "荔湾区", "白云区", "黄埔区"], "510"),
            ("广东省", "东莞市", vec!["南城街道", "东城街道", "莞城街道", "万江街道", "虎门镇"], "523"),
            // 北京市
            ("北京市", "北京市", vec!["朝阳区", "海淀区", "东城区", "西城区", "丰台区", "通州区"], "100"),
            // 上海市
            ("上海市", "上海市", vec!["浦东新区", "黄浦区", "徐汇区", "静安区", "长宁区", "普陀区"], "200"),
            // 浙江省
            ("浙江省", "杭州市", vec!["西湖区", "上城区", "拱墅区", "滨江区", "余杭区", "萧山区"], "310"),
            ("浙江省", "宁波市", vec!["海曙区", "江北区", "鄞州区", "镇海区", "北仑区"], "315"),
            // 江苏省
            ("江苏省", "南京市", vec!["玄武区", "秦淮区", "鼓楼区", "建邺区", "雨花台区", "江宁区"], "210"),
            ("江苏省", "苏州市", vec!["姑苏区", "虎丘区", "吴中区", "相城区", "工业园区"], "215"),
            // 四川省
            ("四川省", "成都市", vec!["武侯区", "锦江区", "青羊区", "金牛区", "成华区", "高新区"], "610"),
            // 湖北省
            ("湖北省", "武汉市", vec!["武昌区", "江汉区", "江岸区", "汉阳区", "洪山区", "青山区"], "430"),
            // 山东省
            ("山东省", "济南市", vec!["历下区", "市中区", "槐荫区", "天桥区", "历城区"], "250"),
            ("山东省", "青岛市", vec!["市南区", "市北区", "李沧区", "崂山区", "城阳区"], "266"),
            // 福建省
            ("福建省", "厦门市", vec!["思明区", "湖里区", "集美区", "海沧区", "同安区"], "361"),
            ("福建省", "福州市", vec!["鼓楼区", "台江区", "仓山区", "晋安区", "马尾区"], "350"),
            // 天津市
            ("天津市", "天津市", vec!["和平区", "河东区", "河西区", "南开区", "河北区", "红桥区"], "300"),
            // 重庆市
            ("重庆市", "重庆市", vec!["渝中区", "江北区", "南岸区", "沙坪坝区", "九龙坡区", "渝北区"], "400"),
        ];
        
        // 街道名称
        let street_names = vec![
            "人民路", "中山路", "解放路", "建设路", "和平路",
            "科技路", "创业路", "高新路", "金融街", "商业街",
            "文化路", "教育路", "体育路", "健康路", "幸福路",
            "滨江路", "湖滨路", "公园路", "花园路", "阳光路",
        ];
        
        // 小区/大厦名称
        let building_names = vec![
            "阳光花园", "翠苑小区", "金色家园", "碧桂园", "万科城",
            "恒大名都", "保利花园", "绿地中心", "华润万家", "龙湖天街",
            "招商局大厦", "腾讯大厦", "科技园", "创业大厦", "金融中心",
        ];
        
        // 随机选择地址信息
        let (province, city, districts, zip_prefix) = &china_addresses[rng.gen_range(0..china_addresses.len())];
        let district = districts[rng.gen_range(0..districts.len())];
        let street_name = street_names[rng.gen_range(0..street_names.len())];
        let building_name = building_names[rng.gen_range(0..building_names.len())];
        
        // 生成门牌号和楼层
        let street_number = rng.gen_range(1..200);
        let building_number = rng.gen_range(1..30);
        let unit_number = rng.gen_range(1..10);
        let room_number = rng.gen_range(101..3099);
        
        // 生成6位邮编
        let postal_code = format!("{}{:03}", zip_prefix, rng.gen_range(0..1000));
        
        // 地址第一行：街道+门牌号+小区名
        let street_address = format!("{}{}号{}", street_name, street_number, building_name);
        // 地址第二行：楼栋+单元+房号
        let street_address_line2 = format!("{}栋{}单元{}", building_number, unit_number, room_number);
        
        BillingAddress {
            street_address,
            street_address_line2,
            city: city.to_string(),
            district: district.to_string(),
            state: province.to_string(),
            postal_code,
            country: "CN".to_string(),
        }
    }

    /// 验证卡号是否符合Luhn算法
    pub fn validate_card_number(card_number: &str) -> bool {
        let digits: Vec<u8> = card_number
            .chars()
            .filter(|c| c.is_numeric())
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();
        
        if digits.len() != 16 {
            return false;
        }
        
        let mut sum = 0;
        let mut double = false;
        
        for &digit in digits.iter().rev() {
            if double {
                let mut doubled = digit * 2;
                if doubled > 9 {
                    doubled = doubled - 9;
                }
                sum += doubled;
            } else {
                sum += digit;
            }
            double = !double;
        }
        
        sum % 10 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_card_number() {
        let card_number = CardGenerator::generate_card_number();
        println!("Generated card number: {}", card_number);
        
        // 验证格式
        assert_eq!(card_number.len(), 19); // 16位数字 + 3个空格
        assert!(card_number.starts_with("6283 19"));
        
        // 验证Luhn算法
        let clean_number = card_number.replace(" ", "");
        assert!(CardGenerator::validate_card_number(&clean_number));
    }

    #[test]
    fn test_generate_expiry_date() {
        let expiry = CardGenerator::generate_expiry_date();
        println!("Generated expiry: {}", expiry);
        
        // 验证格式
        assert_eq!(expiry.len(), 5);
        assert!(expiry.contains("/"));
        
        let parts: Vec<&str> = expiry.split("/").collect();
        assert_eq!(parts.len(), 2);
        
        let month = parts[0].parse::<u32>().unwrap();
        assert!(month >= 1 && month <= 12);
    }

    #[test]
    fn test_generate_cvv() {
        let cvv = CardGenerator::generate_cvv();
        println!("Generated CVV: {}", cvv);
        
        assert_eq!(cvv.len(), 3);
        assert!(cvv.parse::<u32>().unwrap() >= 100);
        assert!(cvv.parse::<u32>().unwrap() < 1000);
    }

    #[test]
    fn test_generate_complete_card() {
        let card = CardGenerator::generate_card();
        println!("Generated card: {:?}", card);
        
        assert!(!card.card_number.is_empty());
        assert!(!card.expiry_date.is_empty());
        assert!(!card.cvv.is_empty());
        assert!(!card.cardholder_name.is_empty());
        assert!(!card.billing_address.street_address.is_empty());
        // street_address_line2可能为空，这是正常的
    }
}
