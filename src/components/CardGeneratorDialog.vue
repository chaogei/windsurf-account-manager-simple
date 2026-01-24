<template>
  <el-dialog
    v-model="visible"
    width="960px"
    :close-on-click-modal="false"
    :show-close="false"
    class="card-generator-dialog"
  >
    <!-- 自定义对话框头部 -->
    <template #header>
      <div class="dialog-header">
        <div class="header-title">
          <div class="header-icon">
            <el-icon><CreditCard /></el-icon>
          </div>
          <h3 class="header-text">{{ $t("dialog.cardGenerator.title") }}</h3>
        </div>
        <el-button
          :icon="Close"
          circle
          @click="visible = false"
          class="close-btn"
        />
      </div>
    </template>

    <div class="card-generator-content">
      <!-- 左侧：BIN管理 -->
      <div class="bin-manager">
        <div class="section-header">
          <div class="section-icon">
            <el-icon><Collection /></el-icon>
          </div>
          <span>{{ $t("dialog.cardGenerator.binManagement") }}</span>
          <el-tag size="small" type="info" class="count-tag">{{
            binList.length
          }}</el-tag>
        </div>

        <div class="bin-add-form">
          <div class="form-row">
            <el-input
              v-model="newBin.prefix"
              :placeholder="$t('dialog.cardGenerator.binPrefix')"
              size="default"
              class="bin-input"
            />
          </div>
          <div class="form-row">
            <el-select
              v-model="newBin.type"
              :placeholder="$t('dialog.cardGenerator.cardType')"
              size="default"
              class="type-select"
            >
              <el-option
                :label="$t('dialog.cardGenerator.types.visa')"
                value="visa"
              />
              <el-option
                :label="$t('dialog.cardGenerator.types.mastercard')"
                value="mastercard"
              />
              <el-option
                :label="$t('dialog.cardGenerator.types.amex')"
                value="amex"
              />
              <el-option
                :label="$t('dialog.cardGenerator.types.discover')"
                value="discover"
              />
              <el-option
                :label="$t('dialog.cardGenerator.types.jcb')"
                value="jcb"
              />
              <el-option
                :label="$t('dialog.cardGenerator.types.unionpay')"
                value="unionpay"
              />
            </el-select>
            <el-button
              type="primary"
              size="default"
              :icon="Plus"
              @click="addBin"
              >{{ $t("dialog.cardGenerator.addBin") }}</el-button
            >
          </div>
        </div>

        <div class="bin-list">
          <TransitionGroup name="bin-list">
            <div
              v-for="(bin, index) in binList"
              :key="bin.prefix"
              class="bin-item"
              :class="[
                { selected: selectedBinIndex === index },
                `card-type-${bin.type}`,
              ]"
              @click="selectBin(index)"
            >
              <div class="card-brand-indicator" :class="bin.type"></div>
              <div class="bin-content">
                <div class="bin-info">
                  <span class="bin-prefix">{{ bin.prefix }}</span>
                  <span class="bin-type-name">{{
                    getCardTypeName(bin.type)
                  }}</span>
                </div>
                <div class="bin-meta">
                  <span class="bin-digits"
                    ><el-icon><Postcard /></el-icon>{{ getCardDigits(bin.type)
                    }}{{ $t("common.digits") || "位" }}</span
                  >
                  <span class="bin-cvc"
                    ><el-icon><Key /></el-icon>{{ getCvcDigits(bin.type)
                    }}{{ $t("common.digits") || "位" }}</span
                  >
                </div>
              </div>
              <el-button
                type="danger"
                :icon="Delete"
                size="small"
                circle
                class="bin-delete"
                @click.stop="removeBin(index)"
              />
            </div>
          </TransitionGroup>

          <div v-if="binList.length === 0" class="empty-state">
            <div class="empty-icon">
              <el-icon><CreditCard /></el-icon>
            </div>
            <p>{{ $t("dialog.cardGenerator.noBin") }}</p>
          </div>
        </div>
      </div>

      <!-- 右侧：生成器 -->
      <div class="generator-panel">
        <div class="section-header">
          <div class="section-icon">
            <el-icon><Cpu /></el-icon>
          </div>
          <span>{{ $t("dialog.cardGenerator.cardGeneration") }}</span>
        </div>

        <div class="generator-options">
          <div class="option-row">
            <span class="option-label">{{
              $t("dialog.cardGenerator.generateCount")
            }}</span>
            <el-input-number
              v-model="generateCount"
              :min="1"
              :max="100"
              size="small"
              class="full-width"
            />
          </div>

          <div class="option-row">
            <span class="option-label">{{
              $t("dialog.cardGenerator.nameFormat")
            }}</span>
            <el-select v-model="nameFormat" size="small" class="full-width">
              <el-option
                :label="$t('dialog.cardGenerator.nameFormats.random')"
                value="random"
              />
              <el-option
                :label="$t('dialog.cardGenerator.nameFormats.custom')"
                value="custom"
              />
            </el-select>
          </div>

          <div class="option-row" v-if="nameFormat === 'custom'">
            <span class="option-label">{{
              $t("dialog.cardGenerator.cardHolderName")
            }}</span>
            <el-input
              v-model="customName"
              :placeholder="$t('dialog.cardGenerator.namePlaceholder')"
              size="small"
              class="full-width"
            />
          </div>

          <div class="option-row">
            <span class="option-label">{{
              $t("dialog.cardGenerator.expiryYear")
            }}</span>
            <el-input-number
              v-model="expiryYear"
              :min="currentYear"
              :max="currentYear + 10"
              size="small"
              class="full-width"
            />
          </div>

          <div class="option-row">
            <span class="option-label">{{
              $t("dialog.cardGenerator.expiryMonth")
            }}</span>
            <el-select v-model="expiryMonth" size="small" class="full-width">
              <el-option
                v-for="m in 12"
                :key="m"
                :label="m.toString().padStart(2, '0')"
                :value="m"
              />
            </el-select>
          </div>
        </div>

        <div class="generate-actions">
          <el-button
            type="primary"
            :icon="Magic"
            @click="generateCards"
            :disabled="selectedBinIndex === null"
            class="generate-btn"
          >
            {{ $t("dialog.cardGenerator.generate") }}
          </el-button>
          <el-button
            :icon="DocumentCopy"
            @click="copyAllCards"
            :disabled="generatedCards.length === 0"
            class="action-btn"
          >
            {{ $t("dialog.cardGenerator.copyAll") }}
          </el-button>
          <el-button
            :icon="Delete"
            @click="clearCards"
            :disabled="generatedCards.length === 0"
            class="action-btn"
          >
            {{ $t("dialog.cardGenerator.clear") }}
          </el-button>
        </div>

        <!-- 生成结果 -->
        <div class="generated-cards">
          <div class="cards-header" v-if="generatedCards.length > 0">
            <div class="cards-header-left">
              <el-icon><Tickets /></el-icon>
              <span>{{
                $t("dialog.cardGenerator.generatedSuccess", {
                  count: generatedCards.length,
                })
              }}</span>
            </div>
            <div class="cards-header-right">
              <el-tag
                v-if="selectedBinIndex !== null"
                size="small"
                :type="getCardTypeTagType(binList[selectedBinIndex].type)"
              >
                {{ getCardTypeName(binList[selectedBinIndex].type) }}
              </el-tag>
            </div>
          </div>

          <div class="cards-list">
            <TransitionGroup name="card-list">
              <div
                v-for="(card, index) in generatedCards"
                :key="card.number"
                class="card-item"
              >
                <div class="card-index">#{{ index + 1 }}</div>
                <div class="card-main">
                  <div class="card-number">
                    {{ formatCardNumber(card.number) }}
                  </div>
                  <div class="card-details">
                    <div class="card-field">
                      <span class="field-label">EXP</span>
                      <span class="field-value">{{ card.expiry }}</span>
                    </div>
                    <div class="card-field">
                      <span class="field-label">CVC</span>
                      <span class="field-value">{{ card.cvc }}</span>
                    </div>
                    <div class="card-field name-field">
                      <span class="field-label">NAME</span>
                      <span class="field-value">{{ card.name }}</span>
                    </div>
                  </div>
                </div>
                <el-button
                  :icon="DocumentCopy"
                  size="small"
                  circle
                  class="copy-btn"
                  @click="copyCard(card)"
                />
              </div>
            </TransitionGroup>

            <div
              v-if="generatedCards.length === 0"
              class="empty-state cards-empty"
            >
              <div class="empty-icon">
                <el-icon><Tickets /></el-icon>
              </div>
              <p>{{ $t("dialog.cardGenerator.noCards") }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage } from "element-plus";
import {
  Plus,
  Delete,
  Collection,
  CreditCard,
  Cpu,
  DocumentCopy,
  Close,
  MagicStick as Magic,
  Postcard,
  Key,
  Tickets,
} from "@element-plus/icons-vue";

const { t } = useI18n();

interface BinConfig {
  prefix: string;
  type: "visa" | "mastercard" | "amex" | "discover" | "jcb" | "unionpay";
}

interface GeneratedCard {
  number: string;
  expiry: string;
  cvc: string;
  name: string;
}

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();

const visible = computed({
  get: () => props.modelValue,
  set: (val) => emit("update:modelValue", val),
});

// BIN管理
const binList = ref<BinConfig[]>([
  { prefix: "411111", type: "visa" },
  { prefix: "521234", type: "mastercard" },
  { prefix: "371234", type: "amex" },
]);

const newBin = ref<BinConfig>({ prefix: "", type: "visa" });
const selectedBinIndex = ref<number | null>(null);

// 生成选项
const generateCount = ref(10);
const nameFormat = ref<"random" | "custom">("random");
const customName = ref("");
const currentYear = new Date().getFullYear();
const expiryYear = ref(currentYear + 3);
const expiryMonth = ref(12);

// 生成结果
const generatedCards = ref<GeneratedCard[]>([]);

// 随机英文名库
const firstNames = [
  "JAMES",
  "JOHN",
  "ROBERT",
  "MICHAEL",
  "WILLIAM",
  "DAVID",
  "RICHARD",
  "JOSEPH",
  "THOMAS",
  "CHARLES",
  "MARY",
  "PATRICIA",
  "JENNIFER",
  "LINDA",
  "ELIZABETH",
  "BARBARA",
  "SUSAN",
  "JESSICA",
  "SARAH",
  "KAREN",
  "NANCY",
  "LISA",
  "BETTY",
  "MARGARET",
  "SANDRA",
  "ASHLEY",
  "KIMBERLY",
  "EMILY",
  "DONNA",
  "MICHELLE",
  "DOROTHY",
  "CAROL",
  "AMANDA",
  "MELISSA",
  "DEBORAH",
  "STEPHANIE",
  "REBECCA",
  "SHARON",
  "LAURA",
  "CYNTHIA",
  "KATHLEEN",
  "AMY",
  "ANGELA",
  "SHIRLEY",
  "ANNA",
];

const lastNames = [
  "SMITH",
  "JOHNSON",
  "WILLIAMS",
  "BROWN",
  "JONES",
  "GARCIA",
  "MILLER",
  "DAVIS",
  "RODRIGUEZ",
  "MARTINEZ",
  "HERNANDEZ",
  "LOPEZ",
  "GONZALEZ",
  "WILSON",
  "ANDERSON",
  "THOMAS",
  "TAYLOR",
  "MOORE",
  "JACKSON",
  "MARTIN",
  "LEE",
  "PEREZ",
  "THOMPSON",
  "WHITE",
  "HARRIS",
  "SANCHEZ",
  "CLARK",
  "RAMIREZ",
  "LEWIS",
  "ROBINSON",
  "WALKER",
  "YOUNG",
  "ALLEN",
  "KING",
  "WRIGHT",
  "SCOTT",
  "TORRES",
  "NGUYEN",
  "HILL",
  "FLORES",
];

// 卡类型配置 (Use computed for i18n names if dynamic, but fixed names are usually fine, or translate them)
const cardTypeConfig = {
  visa: { digits: 16, cvc: 3, name: "Visa" },
  mastercard: { digits: 16, cvc: 3, name: "MasterCard" },
  amex: { digits: 15, cvc: 4, name: "American Express" },
  discover: { digits: 16, cvc: 3, name: "Discover" },
  jcb: { digits: 16, cvc: 3, name: "JCB" },
  unionpay: { digits: 16, cvc: 3, name: "UnionPay" },
};

function getCardTypeName(type: string): string {
  // Try to get from translation, fallback to config name
  return (
    t(`dialog.cardGenerator.types.${type}`) ||
    cardTypeConfig[type as keyof typeof cardTypeConfig]?.name ||
    type
  );
}

function getCardDigits(type: string): number {
  return cardTypeConfig[type as keyof typeof cardTypeConfig]?.digits || 16;
}

function getCvcDigits(type: string): number {
  return cardTypeConfig[type as keyof typeof cardTypeConfig]?.cvc || 3;
}

function getCardTypeTagType(
  type: string,
): "" | "success" | "info" | "warning" | "danger" {
  const types: Record<string, "" | "success" | "info" | "warning" | "danger"> =
    {
      visa: "",
      mastercard: "warning",
      amex: "success",
      discover: "info",
      jcb: "danger",
      unionpay: "",
    };
  return types[type] || "";
}

function addBin() {
  if (!newBin.value.prefix) {
    ElMessage.warning(t("dialog.cardGenerator.enterBinPrefix"));
    return;
  }
  if (!/^\d{4,8}$/.test(newBin.value.prefix)) {
    ElMessage.warning(t("dialog.cardGenerator.invalidBinPrefix"));
    return;
  }
  if (binList.value.some((b) => b.prefix === newBin.value.prefix)) {
    ElMessage.warning(t("dialog.cardGenerator.binExists"));
    return;
  }
  binList.value.push({ ...newBin.value });
  newBin.value = { prefix: "", type: "visa" };
  saveBinList();
  ElMessage.success(t("dialog.cardGenerator.binAdded"));
}

function removeBin(index: number) {
  binList.value.splice(index, 1);
  if (selectedBinIndex.value === index) {
    selectedBinIndex.value = null;
  } else if (
    selectedBinIndex.value !== null &&
    selectedBinIndex.value > index
  ) {
    selectedBinIndex.value--;
  }
  saveBinList();
}

function selectBin(index: number) {
  selectedBinIndex.value = selectedBinIndex.value === index ? null : index;
}

// Luhn算法生成校验位
function luhnChecksum(cardNumber: string): number {
  let sum = 0;
  let shouldDouble = true;

  for (let i = cardNumber.length - 1; i >= 0; i--) {
    let digit = parseInt(cardNumber[i], 10);

    if (shouldDouble) {
      digit *= 2;
      if (digit > 9) {
        digit -= 9;
      }
    }

    sum += digit;
    shouldDouble = !shouldDouble;
  }

  return (10 - (sum % 10)) % 10;
}

// 生成随机数字
function randomDigits(length: number): string {
  let result = "";
  for (let i = 0; i < length; i++) {
    result += Math.floor(Math.random() * 10).toString();
  }
  return result;
}

// 生成随机姓名
function generateRandomName(): string {
  const firstName = firstNames[Math.floor(Math.random() * firstNames.length)];
  const lastName = lastNames[Math.floor(Math.random() * lastNames.length)];
  return `${firstName} ${lastName}`;
}

// 生成卡号
function generateCardNumber(bin: BinConfig): string {
  const config = cardTypeConfig[bin.type];
  const totalDigits = config.digits;
  const remainingDigits = totalDigits - bin.prefix.length - 1; // -1 for checksum

  const partialNumber = bin.prefix + randomDigits(remainingDigits);
  const checksum = luhnChecksum(partialNumber);

  return partialNumber + checksum.toString();
}

// 生成CVC
function generateCvc(type: string): string {
  const cvcLength = getCvcDigits(type);
  return randomDigits(cvcLength);
}

// 生成卡片
function generateCards() {
  if (selectedBinIndex.value === null) {
    ElMessage.warning(t("dialog.cardGenerator.selectBinFirst"));
    return;
  }

  const bin = binList.value[selectedBinIndex.value];
  const expiry = `${expiryMonth.value.toString().padStart(2, "0")}/${(expiryYear.value % 100).toString().padStart(2, "0")}`;

  const cards: GeneratedCard[] = [];
  for (let i = 0; i < generateCount.value; i++) {
    cards.push({
      number: generateCardNumber(bin),
      expiry,
      cvc: generateCvc(bin.type),
      name:
        nameFormat.value === "custom" && customName.value
          ? customName.value.toUpperCase()
          : generateRandomName(),
    });
  }

  generatedCards.value = cards;
  ElMessage.success(
    t("dialog.cardGenerator.generatedSuccess", { count: cards.length }),
  );
}

// 格式化卡号显示
function formatCardNumber(number: string): string {
  if (number.length === 15) {
    // AMEX格式: 4-6-5
    return `${number.slice(0, 4)} ${number.slice(4, 10)} ${number.slice(10)}`;
  }
  // 标准格式: 4-4-4-4
  return number.match(/.{1,4}/g)?.join(" ") || number;
}

// 复制单张卡片
function copyCard(card: GeneratedCard) {
  const text = `${card.number}|${card.expiry}|${card.cvc}|${card.name}`;
  navigator.clipboard.writeText(text);
  ElMessage.success(t("dialog.cardGenerator.copySuccess"));
}

// 复制全部卡片
function copyAllCards() {
  const text = generatedCards.value
    .map((card) => `${card.number}|${card.expiry}|${card.cvc}|${card.name}`)
    .join("\n");
  navigator.clipboard.writeText(text);
  ElMessage.success(
    t("dialog.cardGenerator.copyAllSuccess", {
      count: generatedCards.value.length,
    }),
  );
}

// 清空卡片
function clearCards() {
  generatedCards.value = [];
  ElMessage.success(t("dialog.cardGenerator.cleared"));
}

// 本地存储BIN列表
function saveBinList() {
  localStorage.setItem("card-generator-bins", JSON.stringify(binList.value));
}

function loadBinList() {
  const saved = localStorage.getItem("card-generator-bins");
  if (saved) {
    try {
      binList.value = JSON.parse(saved);
    } catch (e) {
      console.error("Failed to load bin list:", e);
    }
  }
}

onMounted(() => {
  loadBinList();
});
</script>

<style scoped>
/* 对话框基础样式 */
.card-generator-dialog :deep(.el-dialog__body) {
  padding: 0;
}

.card-generator-dialog :deep(.el-dialog__header) {
  padding: 0;
  margin: 0;
}

/* 自定义头部 */
.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
  color: white;
  position: relative;
  overflow: hidden;
}

/* 增加头部光泽感 */
.dialog-header::after {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: radial-gradient(
    circle at top right,
    rgba(255, 255, 255, 0.1) 0%,
    transparent 60%
  );
  pointer-events: none;
}

.header-title {
  display: flex;
  align-items: center;
  gap: 16px;
  z-index: 1;
}

.header-icon {
  width: 48px;
  height: 48px;
  background: rgba(255, 255, 255, 0.15);
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

h3.header-text {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  letter-spacing: 0.5px;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.close-btn {
  background: rgba(255, 255, 255, 0.15) !important;
  border: 1px solid rgba(255, 255, 255, 0.2) !important;
  color: white !important;
  z-index: 1;
  transition: all 0.2s;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.25) !important;
  transform: rotate(90deg);
}

/* 主内容区 */
.card-generator-content {
  display: flex;
  height: 620px;
  background: #f9fafb; /* 极淡的灰背景 */
}

/* 左侧BIN管理 */
.bin-manager {
  width: 320px;
  border-right: 1px solid #e5e7eb;
  display: flex;
  flex-direction: column;
  background: #ffffff;
}

/* BIN添加表单 */
.bin-add-form {
  padding: 16px;
  background: #f9fafb;
  border-bottom: 1px solid #e5e7eb;
}

.form-row {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
}

.form-row:last-child {
  margin-bottom: 0;
}

.bin-input {
  flex: 1;
}

.type-select {
  flex: 1;
}

/* BIN列表 */
.bin-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  background: #f9fafb;
}

/* 右侧生成器 */
.generator-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 24px;
  background: #ffffff;
}

/* 区域头部 */
.section-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  font-weight: 600;
  font-size: 15px;
  color: #374151;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  background: transparent;
}

.section-icon {
  width: 32px;
  height: 32px;
  background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 16px;
  box-shadow: 0 2px 6px rgba(99, 102, 241, 0.2);
}

/* BIN列表项 */
.bin-item {
  position: relative;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  margin-bottom: 10px;
  background: #ffffff;
  border: 1px solid transparent;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.bin-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.06);
  border-color: rgba(99, 102, 241, 0.2);
}

.bin-item.selected {
  background: #eef2ff;
  border-color: #6366f1;
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.15);
}

/* 卡品牌颜色指示条 */
.card-brand-indicator {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  border-radius: 4px 0 0 4px;
}

.card-brand-indicator.visa {
  background: linear-gradient(180deg, #1a1f71, #2b3990);
}
.card-brand-indicator.mastercard {
  background: linear-gradient(180deg, #eb001b, #f79e1b);
}
.card-brand-indicator.amex {
  background: linear-gradient(180deg, #006fcf, #00aeef);
}
.card-brand-indicator.discover {
  background: linear-gradient(180deg, #ff6000, #ffb300);
}
.card-brand-indicator.jcb {
  background: linear-gradient(180deg, #0066b3, #00a94f);
}
.card-brand-indicator.unionpay {
  background: linear-gradient(180deg, #e21836, #00447c);
}

.bin-content {
  flex: 1;
  min-width: 0;
  margin-left: 8px;
}

.bin-info {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 4px;
}

.bin-prefix {
  font-family: "Monaco", "Menlo", monospace;
  font-weight: 700;
  font-size: 15px;
  color: #1f2937;
  letter-spacing: 1px;
}

.bin-type-name {
  font-size: 12px;
  color: #6b7280;
  background: #f3f4f6;
  padding: 2px 8px;
  border-radius: 4px;
}

.bin-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: #9ca3af;
}

.bin-meta span {
  display: flex;
  align-items: center;
  gap: 4px;
}

.bin-meta .el-icon {
  font-size: 12px;
}

.bin-delete {
  opacity: 0;
  transition: all 0.2s;
  flex-shrink: 0;
}

.bin-item:hover .bin-delete {
  opacity: 1;
}

/* 生成选项 */
.generator-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px 24px;
  padding: 20px 24px;
  background: #f9fafb;
  background: #f9fafb;
  border-radius: 16px;
  margin-bottom: 24px;
  border: 1px solid #f3f4f6;
}

.option-row {
  display: flex;
  flex-direction: column;
}

.option-label {
  font-size: 13px;
  color: #6b7280;
  font-weight: 600;
  margin-bottom: 8px;
}

.full-width {
  width: 100% !important;
}

/* 生成按钮 */
.generate-btn {
  background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%) !important;
  border: none !important;
  font-weight: 600;
  padding: 10px 24px;
  height: 40px;
  letter-spacing: 0.5px;
}

.generate-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(79, 70, 229, 0.35);
  filter: brightness(1.1);
}

/* 生成结果区 */
.generated-cards {
  flex: 1;
  display: flex;
  flex-direction: column;
  border: 1px solid #e5e7eb;
  border-radius: 16px;
  overflow: hidden;
  background: #ffffff;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
}

.cards-header {
  padding: 14px 20px;
  background: #f9fafb;
  font-size: 13px;
  font-weight: 600;
  color: #374151;
  border-bottom: 1px solid #e5e7eb;
}

.cards-header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.cards-header-left .el-icon {
  color: var(--el-color-primary);
}

.cards-header-left strong {
  color: var(--el-color-primary);
  font-size: 15px;
}

.cards-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

/* 卡片项 */
.card-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 16px;
  margin-bottom: 10px;
  background: #ffffff;
  border: 1px solid #e5e7eb;
  border-radius: 10px;
  transition: all 0.2s ease;
}

.card-item:hover {
  border-color: #818cf8;
  background: #f5f3ff;
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.1);
  transform: translateX(2px);
}

.card-item:active {
  transform: translateX(0);
  box-shadow: 0 2px 6px rgba(99, 102, 241, 0.15);
}

.card-index {
  width: 38px;
  height: 38px;
  background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 700;
  color: white;
  flex-shrink: 0;
  box-shadow: 0 2px 6px rgba(99, 102, 241, 0.25);
}

.card-main {
  flex: 1;
  min-width: 0;
}

.card-number {
  font-family: "Monaco", "Menlo", "Consolas", monospace;
  font-weight: 700;
  font-size: 15px;
  color: var(--el-text-color-primary);
  letter-spacing: 2px;
  margin-bottom: 6px;
}

.card-details {
  display: flex;
  gap: 20px;
  align-items: center;
}

.card-field {
  display: flex;
  align-items: center;
  gap: 6px;
}

.field-label {
  font-size: 10px;
  font-weight: 600;
  color: var(--el-text-color-placeholder);
  text-transform: uppercase;
}

.field-value {
  font-family: "Monaco", "Menlo", "Consolas", monospace;
  font-size: 13px;
  color: var(--el-text-color-regular);
}

.name-field {
  flex: 1;
}

.name-field .field-value {
  text-transform: uppercase;
  font-size: 12px;
}

.copy-btn {
  opacity: 0.5;
  transition: all 0.2s;
  color: #9ca3af !important;
  border-color: #e5e7eb !important;
  background: transparent !important;
}

.card-item:hover .copy-btn {
  opacity: 1;
  color: #6366f1 !important;
  border-color: #c7d2fe !important;
  background: #eef2ff !important;
}

.copy-btn:hover {
  color: white !important;
  border-color: #6366f1 !important;
  background: #6366f1 !important;
  transform: scale(1.1);
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  color: var(--el-text-color-secondary);
}

.empty-icon {
  width: 64px;
  height: 64px;
  background: var(--el-fill-color-light);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  color: var(--el-text-color-placeholder);
  margin-bottom: 12px;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

.cards-empty {
  height: 100%;
}

/* 列表动画 */
.bin-list-enter-active,
.bin-list-leave-active,
.card-list-enter-active,
.card-list-leave-active {
  transition: all 0.3s ease;
}

.bin-list-enter-from,
.card-list-enter-from {
  opacity: 0;
  transform: translateX(-20px);
}

.bin-list-leave-to,
.card-list-leave-to {
  opacity: 0;
  transform: translateX(20px);
}

/* 自定义滚动条 */
.bin-list::-webkit-scrollbar,
.cards-list::-webkit-scrollbar {
  width: 8px;
}

.bin-list::-webkit-scrollbar-track,
.cards-list::-webkit-scrollbar-track {
  background: #f3f4f6;
  border-radius: 4px;
}

.bin-list::-webkit-scrollbar-thumb,
.cards-list::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 4px;
  border: 2px solid #f3f4f6;
}

.bin-list::-webkit-scrollbar-thumb:hover,
.cards-list::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

/* 暗色模式滚动条 */
:root.dark .bin-list::-webkit-scrollbar-track,
:root.dark .cards-list::-webkit-scrollbar-track {
  background: #1f2937;
}

:root.dark .bin-list::-webkit-scrollbar-thumb,
:root.dark .cards-list::-webkit-scrollbar-thumb {
  background: #4b5563;
  border-color: #1f2937;
}

:root.dark .bin-list::-webkit-scrollbar-thumb:hover,
:root.dark .cards-list::-webkit-scrollbar-thumb:hover {
  background: #6b7280;
}

/* 暗色模式适配 */
:root.dark .dialog-header {
  background: linear-gradient(135deg, #4a5568 0%, #2d3748 100%);
}

:root.dark .card-generator-content {
  background: var(--el-bg-color-overlay);
}

:root.dark .bin-manager {
  background: #1f2937;
}

:root.dark .bin-add-form {
  background: #111827;
  border-color: #374151;
}

:root.dark .bin-list {
  background: #111827;
}

:root.dark .bin-item {
  background: #1f2937;
  border-color: #374151;
}

:root.dark .bin-item:hover {
  background: #374151;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
}

:root.dark .bin-item.selected {
  background: rgba(99, 102, 241, 0.15);
  border-color: #6366f1;
}

:root.dark .bin-prefix {
  color: #f3f4f6;
}

:root.dark .bin-type-name {
  background: #374151;
  color: #d1d5db;
}

:root.dark .generator-panel {
  background: #111827;
}

:root.dark .section-header {
  color: #f3f4f6;
}

:root.dark .generator-options {
  background: #1f2937;
  border-color: #374151;
}

:root.dark .generated-cards {
  background: #1f2937;
  border-color: #374151;
}

:root.dark .cards-header {
  background: #111827;
  border-color: #374151;
  color: #f3f4f6;
}

:root.dark .card-item {
  background: #1f2937;
  border-color: #374151;
}

:root.dark .card-item:hover {
  background: #374151;
  border-color: #6366f1;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
}

:root.dark .card-number {
  color: #f3f4f6;
}

:root.dark .field-value {
  color: #d1d5db;
}

:root.dark .copy-btn {
  color: #6b7280 !important;
  border-color: #374151 !important;
}

:root.dark .card-item:hover .copy-btn {
  color: #818cf8 !important;
  border-color: #4f46e5 !important;
  background: rgba(99, 102, 241, 0.2) !important;
}

:root.dark .copy-btn:hover {
  color: white !important;
  background: #6366f1 !important;
}

:root.dark .empty-icon {
  background: #374151;
  color: #6b7280;
}

:root.dark .generate-btn {
  background: linear-gradient(135deg, #5a67d8 0%, #6b46c1 100%) !important;
}
</style>
