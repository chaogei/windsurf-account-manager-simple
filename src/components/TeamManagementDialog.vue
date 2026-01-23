<template>
  <el-dialog
    v-model="dialogVisible"
    :title="$t('dialog.teamManagement.title')"
    width="900px"
    :close-on-click-modal="false"
    destroy-on-close
    class="team-management-dialog"
  >
    <div v-loading="loading" class="team-container">
      <!-- 标签页 -->
      <el-tabs v-model="activeTab" type="border-card">
        <!-- 团队成员列表 -->
        <el-tab-pane :label="$t('dialog.teamManagement.members')" name="members">
          <!-- 邀请链接区域 -->
          <div v-if="teamInviteId" class="invite-link-section">
            <div class="invite-link-label">
              <el-icon><Link /></el-icon>
              <span>{{ $t('dialog.teamManagement.inviteId') }}:</span>
            </div>
            <div class="invite-link-content">
              <el-input
                v-model="teamInviteId"
                readonly
                size="small"
                class="invite-id-input"
              />
              <el-button type="primary" size="small" @click="copyInviteId">
                <el-icon><CopyDocument /></el-icon>
                {{ $t('common.copy') }}
              </el-button>
              <el-button size="small" @click="copyInviteUrl">
                <el-icon><Link /></el-icon>
                {{ $t('dialog.teamManagement.copyLink') }}
              </el-button>
            </div>
          </div>
          
          <div class="tab-header">
            <el-button type="primary" size="small" @click="showInviteDialog = true">
              <el-icon><Plus /></el-icon>
              {{ $t('dialog.teamManagement.inviteMember') }}
            </el-button>
            <el-button 
              type="warning" 
              size="small" 
              :loading="batchResettingCredits"
              :disabled="otherMembers.length === 0"
              @click="batchResetMemberCredits"
            >
              <el-icon><RefreshRight /></el-icon>
              {{ $t('dialog.teamManagement.batchResetCredits') }}
            </el-button>
            <el-button size="small" @click="loadTeamMembers">
              <el-icon><Refresh /></el-icon>
              {{ $t('common.refresh') }}
            </el-button>
            <el-button type="danger" size="small" @click="showTransferDialog = true">
              <el-icon><Switch /></el-icon>
              {{ $t('dialog.teamManagement.transferSubscription') }}
            </el-button>
          </div>
          
          <el-table :data="members" style="width: 100%" max-height="400" class="member-table">
            <el-table-column :label="$t('dialog.teamManagement.nameAndEmail')" min-width="220">
              <template #default="{ row }">
                <div class="member-cell">
                  <div class="member-cell-name">
                    {{ row.name }}
                    <el-tag v-if="row.role === 'Admin'" type="warning" size="small" class="role-tag">Admin</el-tag>
                  </div>
                  <div class="member-cell-email">{{ row.email }}</div>
                </div>
              </template>
            </el-table-column>
            <el-table-column :label="$t('dialog.teamManagement.lastUsed')" width="120" align="center">
              <template #default="{ row }">
                <span class="time-text">{{ formatLastUsed(row.last_update_time) }}</span>
              </template>
            </el-table-column>
            <el-table-column :label="$t('dialog.teamManagement.usedCredits')" width="100" align="center">
              <template #default="{ row }">
                <span>{{ Math.floor((row.prompts_used || 0) / 100) }}</span>
              </template>
            </el-table-column>
            <el-table-column :label="$t('dialog.teamManagement.disableAccess')" width="90" align="center">
              <template #default="{ row }">
                <el-tag :type="row.disable_codeium ? 'danger' : 'success'" size="small">
                  {{ row.disable_codeium ? $t('common.yes') : $t('common.no') }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="$t('dialog.teamManagement.actions')" width="180" fixed="right" align="center">
              <template #default="{ row }">
                <el-button type="info" size="small" text @click="openMemberDetail(row)">
                  {{ $t('dialog.teamManagement.edit') }}
                </el-button>
                <template v-if="row.role !== 'Admin'">
                  <el-popconfirm
                    :title="$t('dialog.teamManagement.confirmRemoveMember')"
                    :confirm-button-text="$t('dialog.teamManagement.yes')"
                    :cancel-button-text="$t('dialog.teamManagement.no')"
                    @confirm="removeMember(row)"
                  >
                    <template #reference>
                      <el-button type="danger" size="small" text>
                        {{ $t('dialog.teamManagement.remove') }}
                      </el-button>
                    </template>
                  </el-popconfirm>
                  <el-button type="primary" size="small" text :loading="row.rejoining" @click="rejoinMember(row)">
                    {{ $t('dialog.teamManagement.resetCredits') }}
                  </el-button>
                </template>
              </template>
            </el-table-column>
          </el-table>
          
          <div v-if="members.length === 0 && !loading" class="empty-state">
            <el-empty :description="$t('dialog.teamManagement.noMembers')" />
          </div>
        </el-tab-pane>
        
        <!-- 待处理邀请 -->
        <el-tab-pane :label="$t('dialog.teamManagement.pendingInvitations')" name="invitations">
          <div class="tab-header">
            <el-button size="small" @click="loadPendingInvitations">
              <el-icon><Refresh /></el-icon>
              {{ $t('common.refresh') }}
            </el-button>
          </div>
          
          <el-table :data="pendingInvitations" style="width: 100%" max-height="400">
            <el-table-column prop="name" :label="$t('dialog.teamManagement.name')" width="150" />
            <el-table-column prop="email" :label="$t('dialog.teamManagement.email')" min-width="200" />
            <el-table-column prop="created_at" :label="$t('dialog.teamManagement.invitationTime')" width="180">
              <template #default="{ row }">
                {{ formatTime(row.created_at) }}
              </template>
            </el-table-column>
            <el-table-column :label="$t('dialog.teamManagement.actions')" width="100" fixed="right">
              <template #default="{ row }">
                <el-popconfirm
                  :title="$t('dialog.teamManagement.confirmRevokeInvitation')"
                  :confirm-button-text="$t('dialog.teamManagement.yes')"
                  :cancel-button-text="$t('dialog.teamManagement.no')"
                  @confirm="revokeInvitation(row)"
                >
                  <template #reference>
                    <el-button type="warning" size="small" text>
                      {{ $t('dialog.teamManagement.revokeInvitation') }}
                    </el-button>
                  </template>
                </el-popconfirm>
              </template>
            </el-table-column>
          </el-table>
          
          <div v-if="pendingInvitations.length === 0 && !loading" class="empty-state">
            <el-empty :description="$t('dialog.teamManagement.noPendingInvitations')" />
          </div>
        </el-tab-pane>
        
        <!-- 我的邀请（普通用户） -->
        <el-tab-pane :label="$t('dialog.teamManagement.myInvitation')" name="my-invitation">
          <div class="my-invitation-section">
            <div v-if="myInvitation" class="invitation-card">
              <div class="invitation-info">
                <h3>{{ $t('dialog.teamManagement.youReceivedInvitation') }}</h3>
                <p><strong>{{ $t('dialog.teamManagement.teamName') }}:</strong> {{ myInvitation.team_name || $t('dialog.teamManagement.unknownTeam') }}</p>
                <p><strong>{{ $t('dialog.teamManagement.inviter') }}:</strong> {{ myInvitation.admin_name || $t('dialog.teamManagement.admin') }}</p>
              </div>
              <div class="invitation-actions">
                <el-button type="primary" @click="acceptInvitation">
                  {{ $t('dialog.teamManagement.acceptInvitation') }}
                </el-button>
                <el-button type="danger" @click="rejectInvitation">
                  {{ $t('dialog.teamManagement.rejectInvitation') }}
                </el-button>
              </div>
            </div>
            <div v-else class="empty-state">
              <el-empty :description="$t('dialog.teamManagement.noPendingInvitations')" />
              <el-button size="small" @click="loadMyInvitation">
                <el-icon><Refresh /></el-icon>
                {{ $t('common.checkInvitation') }}
              </el-button>
            </div>
          </div>
        </el-tab-pane>

        <!-- 申请加入团队 -->
        <el-tab-pane :label="$t('dialog.teamManagement.joinTeam')" name="join-team">
          <div class="join-team-section">
            <el-alert
              title="{{ $t('dialog.teamManagement.joinTeam') }}"
              type="info"
              description="{{ $t('dialog.teamManagement.joinTeamDescription') }}"
              :closable="false"
              show-icon
              style="margin-bottom: 20px"
            />
            <el-form :model="joinForm" label-width="100px">
              <el-form-item :label="$t('dialog.teamManagement.inviteId')">
                <el-input
                  v-model="joinForm.inviteId"
                  placeholder="{{ $t('dialog.teamManagement.inviteIdPlaceholder') }}"
                  clearable
                />
              </el-form-item>
              <el-form-item>
                <el-button type="primary" :loading="joining" @click="submitJoinRequest">
                  {{ $t('dialog.teamManagement.submitJoinRequest') }}
                </el-button>
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>

        <!-- 待审批申请（管理员） -->
        <el-tab-pane label="待审批" name="pending-requests">
          <div class="tab-header">
            <el-button size="small" @click="loadTeamMembers">
              <el-icon><Refresh /></el-icon>
              {{ $t('common.refresh') }}
            </el-button>
          </div>
          
          <el-table :data="pendingMembers" style="width: 100%" max-height="400">
            <el-table-column prop="name" :label="$t('dialog.teamManagement.name')" width="150" />
            <el-table-column prop="email" :label="$t('dialog.teamManagement.email')" min-width="200" />
            <el-table-column prop="status" :label="$t('dialog.teamManagement.status')" width="100">
              <template #default>
                <el-tag type="warning" size="small">{{ $t('dialog.teamManagement.pending') }}</el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="$t('dialog.teamManagement.actions')" width="160" fixed="right">
              <template #default="{ row }">
                <el-button type="success" size="small" text @click="approveJoinRequest(row, 'approve')">
                  {{ $t('dialog.teamManagement.approve') }}
                </el-button>
                <el-button type="danger" size="small" text @click="approveJoinRequest(row, 'reject')">
                  {{ $t('dialog.teamManagement.reject') }}
                </el-button>
              </template>
            </el-table-column>
          </el-table>
          
          <div v-if="pendingMembers.length === 0 && !loading" class="empty-state">
            <el-empty :description="$t('dialog.teamManagement.noPendingRequests')" />
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>
    
    <!-- 邀请成员对话框 -->
    <el-dialog
      v-model="showInviteDialog"
      :title="$t('dialog.teamManagement.inviteMember')"
      width="500px"
      :close-on-click-modal="false"
      append-to-body
    >
      <el-form :model="inviteForm" label-width="60px">
        <div v-for="(user, index) in inviteForm.users" :key="index" class="invite-user-row">
          <div class="invite-user-fields">
            <el-form-item :label="$t('dialog.teamManagement.name')">
              <el-input v-model="user.name" :placeholder="$t('dialog.teamManagement.memberName')" />
            </el-form-item>
            <el-form-item :label="$t('dialog.teamManagement.email')">
              <el-input v-model="user.email" :placeholder="$t('dialog.teamManagement.memberEmail')" />
            </el-form-item>
          </div>
          <el-button
            v-if="inviteForm.users.length > 1 && index > 0"
            type="danger"
            :icon="Delete"
            circle
            size="small"
            class="delete-btn"
            @click="removeInviteUser(index)"
          />
        </div>
        <el-button class="add-more-btn" @click="addInviteUser">
          <el-icon><Plus /></el-icon>
          {{ $t('dialog.teamManagement.addMore') }}
        </el-button>
        
        <!-- 自动加入开关 -->
        <div class="auto-join-section">
          <el-switch v-model="autoJoinEnabled" />
          <span class="auto-join-label">{{ $t('dialog.teamManagement.autoJoin') }}</span>
          <el-tooltip content="邀请后，如果成员邮箱在账号管理器中，将自动接受邀请加入团队" placement="top">
            <el-icon class="help-icon"><QuestionFilled /></el-icon>
          </el-tooltip>
        </div>
      </el-form>
      <template #footer>
        <el-button @click="showInviteDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" :loading="inviting" @click="submitInvite">
          {{ autoJoinEnabled ? $t('dialog.teamManagement.inviteAndAutoJoin') : $t('dialog.teamManagement.sendInvite') }}
        </el-button>
      </template>
    </el-dialog>
    
    <!-- 转让订阅对话框 -->
    <el-dialog
      v-model="showTransferDialog"
      :title="$t('dialog.teamManagement.transferSubscription')"
      width="500px"
      :close-on-click-modal="false"
      append-to-body
    >
      <el-alert
        :title="$t('dialog.teamManagement.transferSubscriptionDesc')"
        type="warning"
        :description="$t('dialog.teamManagement.transferWarning')"
        :closable="false"
        show-icon
        style="margin-bottom: 20px"
      />
      <el-form :model="transferForm" label-width="100px" autocomplete="off">
        <el-form-item :label="$t('dialog.teamManagement.targetEmail')" required>
          <el-input
            v-model="transferForm.email"
            :placeholder="$t('dialog.teamManagement.targetEmailPlaceholder')"
            clearable
            name="transfer-target-email-no-autofill"
            autocomplete="off"
            data-form-type="other"
          />
        </el-form-item>
        <el-form-item :label="$t('dialog.teamManagement.userName')">
          <el-input
            v-model="transferForm.name"
            :placeholder="$t('dialog.teamManagement.userNameOptional')"
            clearable
            name="transfer-target-name-no-autofill"
            autocomplete="off"
            data-form-type="other"
          />
        </el-form-item>
      </el-form>
      
      <!-- 转让进度显示 -->
      <div v-if="transferring" class="transfer-progress">
        <el-steps :active="transferStep" finish-status="success" simple>
          <el-step title="禁用访问" />
          <el-step title="邀请用户" />
          <el-step title="授予管理员" />
          <el-step title="移除自己" />
        </el-steps>
        <div class="transfer-status">{{ transferStatus }}</div>
      </div>
      
      <template #footer>
        <el-button @click="showTransferDialog = false" :disabled="transferring">取消</el-button>
        <el-button type="danger" :loading="transferring" @click="executeTransfer">
          确认转让
        </el-button>
      </template>
    </el-dialog>

    <!-- 成员详情对话框 -->
    <el-dialog
      v-model="showMemberDetail"
      :title="selectedMember?.name || '成员详情'"
      width="520px"
      :close-on-click-modal="false"
      append-to-body
      class="member-detail-dialog"
    >
      <div v-if="selectedMember" class="member-detail-content">
        <!-- 用户基本信息卡片 -->
        <div class="info-card">
          <div class="info-header">
            <el-avatar :size="48" class="member-avatar">
              {{ selectedMember.name?.charAt(0)?.toUpperCase() || '?' }}
            </el-avatar>
            <div class="member-info">
              <div class="member-name">{{ selectedMember.name }}</div>
              <div class="member-email">{{ selectedMember.email }}</div>
            </div>
          </div>
        </div>
        
        <el-divider />
        
        <el-form label-width="120px" class="detail-form">
          <el-form-item label="API Key">
            <el-input 
              :value="selectedMember.api_key" 
              readonly 
              size="small"
              class="api-key-input"
            >
              <template #append>
                <el-tooltip content="复制" placement="top">
                  <el-button :icon="CopyDocument" @click="copyApiKey" />
                </el-tooltip>
              </template>
            </el-input>
          </el-form-item>
          
          <el-form-item label="注册时间">
            <span class="info-value">{{ formatSignUpTime(selectedMember.sign_up_time) }}</span>
          </el-form-item>
          
          <el-form-item label="角色">
            <el-select v-model="memberDetailForm.role" style="width: 200px" size="default">
              <el-option label="普通用户" value="User" />
              <el-option label="管理员" value="Admin" />
            </el-select>
          </el-form-item>
          
          <el-form-item label="禁用访问">
            <el-switch 
              v-model="memberDetailForm.disableAccess"
              active-text="已禁用"
              inactive-text=""
              style="--el-switch-on-color: #f56c6c"
            />
            <div class="form-tip">禁用后该成员将无法使用 Windsurf，且不占用席位</div>
          </el-form-item>
        </el-form>
      </div>
      
      <template #footer>
        <div class="dialog-footer">
          <el-button type="danger" plain @click="handleRemoveMember" :loading="memberDetailLoading">
            移除成员
          </el-button>
          <el-button type="primary" @click="saveMemberDetail" :loading="memberDetailLoading">
            保存修改
          </el-button>
        </div>
      </template>
    </el-dialog>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Refresh, Delete, Link, CopyDocument, QuestionFilled, RefreshRight, Switch } from '@element-plus/icons-vue'

interface Props {
  modelValue: boolean
  accountId: string
}

const props = defineProps<Props>()
const emit = defineEmits(['update:modelValue'])

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

const loading = ref(false)
const activeTab = ref('members')

// 团队邀请ID
const teamInviteId = ref('')

// 团队成员数据
interface TeamMember {
  api_key: string
  name: string
  email: string
  role?: string
  sign_up_time?: number
  last_update_time?: number
  prompts_used?: number
  disable_codeium?: boolean
}
const members = ref<TeamMember[]>([])

// 成员详情对话框
const showMemberDetail = ref(false)
const selectedMember = ref<TeamMember | null>(null)
const memberDetailLoading = ref(false)
const memberDetailForm = ref({
  role: 'User',
  disableAccess: false
})

// 待处理邀请
interface PendingInvitation {
  id: string
  name: string
  email: string
  created_at?: number
}
const pendingInvitations = ref<PendingInvitation[]>([])

// 我的邀请
interface MyInvitation {
  approval_id: string
  team_name?: string
  admin_name?: string
}
const myInvitation = ref<MyInvitation | null>(null)

// 邀请表单
const showInviteDialog = ref(false)
const inviting = ref(false)
const inviteForm = ref({
  users: [{ name: '', email: '' }]
})
const autoJoinEnabled = ref(true)  // 自动加入开关，默认开启

// 申请加入团队
const joining = ref(false)
const joinForm = ref({
  inviteId: ''
})

// 待审批成员（team_status = PENDING）
const pendingMembers = ref<TeamMember[]>([])

// 批量重置积分状态
const batchResettingCredits = ref(false)

// 转让订阅相关
const showTransferDialog = ref(false)
const transferring = ref(false)
const transferStep = ref(0)
const transferStatus = ref('')
const transferForm = ref({
  email: '',
  name: ''
})

// 当前账号邮箱（用于排除自己）
const currentAccountEmail = ref('')

// 计算其他成员列表（排除自己）
const otherMembers = computed(() => {
  return members.value.filter(m => m.email?.toLowerCase() !== currentAccountEmail.value?.toLowerCase())
})

// 加载团队成员
async function loadTeamMembers() {
  if (!props.accountId) return
  
  loading.value = true
  try {
    const result = await invoke<any>('get_team_members', {
      id: props.accountId,
      groupId: null
    })

    if (result.success) {
      const data = result.data || {}
      
      // subMesssage_1 是 User[] 数组（可能是单个对象）
      let users = data.subMesssage_1 || []
      // 如果是单个对象，转换为数组
      if (users && !Array.isArray(users)) {
        users = [users]
      }
      // subMesssage_2 是 UserRole[] 数组（可能是单个对象）
      let userRoles = data.subMesssage_2 || []
      if (userRoles && !Array.isArray(userRoles)) {
        userRoles = [userRoles]
      }
      // subMesssage_4 是 UserCascadeDetails，可能是数组或单个对象
      const cascadeDetails = data.subMesssage_4 || []
      
      // 构建成员列表
      const approvedList: TeamMember[] = []
      const pendingList: TeamMember[] = []
      
      // 遍历用户数组
      if (Array.isArray(users) && users.length > 0) {
        for (const user of users) {
          const apiKey = user.string_1 || ''
          const firebaseId = user.string_6 || ''
          const name = user.string_2 || ''
          const email = user.string_3 || ''
          const teamStatus = user.int_8 || 0
          // signup_time: field 4 是 Timestamp，其 seconds 在 subMesssage_4.int_1
          const signUpTime = user.subMesssage_4?.int_1 || 0
          // last_update_time: field 26 是 Timestamp
          const lastUpdateTime = user.subMesssage_26?.int_1 || 0
          // disable_codeium: field 16，bool 类型，解析为 int_16
          const disableCodeium = user.int_16 === 1
          
          // 查找角色
          let role = 'User'
          if (Array.isArray(userRoles)) {
            const roleInfo = userRoles.find((r: any) => r.string_1 === apiKey)
            if (roleInfo) {
              role = roleInfo.string_4 || roleInfo.string_2 || 'User'
            }
          }
          
          // 查找使用量（通过 firebase_id 关联）
          let promptsUsed = 0
          if (Array.isArray(cascadeDetails)) {
            // 如果是数组
            const usageInfo = cascadeDetails.find((c: any) => c.string_1 === firebaseId)
            if (usageInfo) {
              promptsUsed = usageInfo.int_2 || 0
            }
          } else if (cascadeDetails && typeof cascadeDetails === 'object') {
            // 如果是单个对象
            if (cascadeDetails.string_1 === firebaseId) {
              promptsUsed = cascadeDetails.int_2 || 0
            }
          }
          
          const member: TeamMember = { 
            api_key: apiKey, 
            name, 
            email, 
            role,
            sign_up_time: signUpTime,
            last_update_time: lastUpdateTime,
            prompts_used: promptsUsed,
            disable_codeium: disableCodeium
          }
          
          // 根据 team_status 分类：1=PENDING, 2=APPROVED
          if (teamStatus === 1) {
            pendingList.push(member)
          } else {
            approvedList.push(member)
          }
        }
      }
      
      // 排序：管理员排在最前面
      approvedList.sort((a, b) => {
        const aRole = typeof a.role === 'string' ? a.role.toLowerCase() : ''
        const bRole = typeof b.role === 'string' ? b.role.toLowerCase() : ''
        const aIsAdmin = aRole === 'admin' ? 0 : 1
        const bIsAdmin = bRole === 'admin' ? 0 : 1
        return aIsAdmin - bIsAdmin
      })
      
      members.value = approvedList
      pendingMembers.value = pendingList
    } else {
      ElMessage.error(result.error || '获取团队成员失败')
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    loading.value = false
  }
}

// 移除成员
async function removeMember(member: TeamMember) {
  loading.value = true
  try {
    const result = await invoke<any>('remove_team_member', {
      id: props.accountId,
      memberApiKey: member.api_key
    })
    
    if (result.success) {
      ElMessage.success('成员已移除')
      loadTeamMembers()
    } else {
      ElMessage.error(result.error || '移除成员失败')
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    loading.value = false
  }
}

// ==================== 成员详情相关 ====================

// 打开成员详情对话框
function openMemberDetail(member: TeamMember) {
  selectedMember.value = member
  // 角色映射：确保值与 el-option 的 value 匹配
  let role = typeof member.role === 'string' ? member.role : 'User'
  if (role.toLowerCase() === 'admin') role = 'Admin'
  else role = 'User'
  
  memberDetailForm.value = {
    role: role,
    disableAccess: member.disable_codeium || false
  }
  showMemberDetail.value = true
}

// 复制API Key
function copyApiKey() {
  if (!selectedMember.value) return
  navigator.clipboard.writeText(selectedMember.value.api_key)
    .then(() => ElMessage.success('已复制 API Key'))
    .catch(() => ElMessage.error('复制失败'))
}

// 格式化注册时间
function formatSignUpTime(timestamp?: number): string {
  if (!timestamp) return '未知'
  const date = new Date(timestamp * 1000)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  })
}

// 格式化最后使用时间（相对时间）
function formatLastUsed(timestamp?: number): string {
  if (!timestamp) return '未使用'
  
  const now = Date.now()
  const time = timestamp * 1000
  const diff = now - time
  
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)
  
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  if (days < 30) return `${days}天前`
  
  return new Date(time).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
}

// 保存成员详情（角色和访问权限）
async function saveMemberDetail() {
  if (!selectedMember.value) return
  
  memberDetailLoading.value = true
  try {
    // 标准化角色值进行比较
    let originalRole = typeof selectedMember.value.role === 'string' ? selectedMember.value.role : 'User'
    if (originalRole.toLowerCase() === 'admin') originalRole = 'Admin'
    else originalRole = 'User'
    
    const newRole = memberDetailForm.value.role
    const originalDisabled = selectedMember.value.disable_codeium || false
    const newDisabled = memberDetailForm.value.disableAccess
    
    // 更新角色（如果有变化）
    if (originalRole !== newRole) {
      // 先移除旧角色（使用 root.xxx 格式）
      if (originalRole !== 'User') {
        const oldRoleApi = originalRole === 'Admin' ? 'root.admin' : `root.${originalRole.toLowerCase()}`
        await invoke<any>('remove_user_role', {
          id: props.accountId,
          memberApiKey: selectedMember.value.api_key,
          role: oldRoleApi
        })
      }
      // 添加新角色（使用 root.xxx 格式）
      if (newRole !== 'User') {
        const newRoleApi = newRole === 'Admin' ? 'root.admin' : `root.${newRole.toLowerCase()}`
        await invoke<any>('add_user_role', {
          id: props.accountId,
          memberApiKey: selectedMember.value.api_key,
          role: newRoleApi
        })
      }
    }
    
    // 更新访问权限（如果有变化）
    if (originalDisabled !== newDisabled) {
      await invoke<any>('update_codeium_access', {
        id: props.accountId,
        memberApiKey: selectedMember.value.api_key,
        disableAccess: newDisabled
      })
    }
    
    ElMessage.success('保存成功')
    showMemberDetail.value = false
    loadTeamMembers()
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    memberDetailLoading.value = false
  }
}

// 从详情对话框移除成员
async function handleRemoveMember() {
  if (!selectedMember.value) return
  
  try {
    await ElMessageBox.confirm('确定要移除该成员吗？', '确认', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    memberDetailLoading.value = true
    const result = await invoke<any>('remove_team_member', {
      id: props.accountId,
      memberApiKey: selectedMember.value.api_key
    })
    
    if (result.success) {
      ElMessage.success('成员已移除')
      showMemberDetail.value = false
      loadTeamMembers()
    } else {
      ElMessage.error(result.error || '移除成员失败')
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error(error.toString())
    }
  } finally {
    memberDetailLoading.value = false
  }
}

// 重新加入成员（移除 → 邀请 → 自动接受）
async function rejoinMember(member: TeamMember) {
  loading.value = true
  try {
    // Step 1: 移除成员
    const removeResult = await invoke<any>('remove_team_member', {
      id: props.accountId,
      memberApiKey: member.api_key
    })
    
    if (!removeResult.success) {
      ElMessage.error(removeResult.error || '移除成员失败')
      return
    }
    
    // Step 2: 重新邀请
    const inviteResult = await invoke<any>('invite_team_members', {
      id: props.accountId,
      users: [{ name: member.name, email: member.email }]
    })
    
    if (!inviteResult.success) {
      ElMessage.error(inviteResult.error || '重新邀请失败')
      loadTeamMembers()
      return
    }
    
    // Step 3: 自动接受邀请（如果邮箱在管理器中）
    const autoJoinResults = await autoAcceptInvitations([member.email])
    const joined = autoJoinResults.some(r => r.success)
    
    if (joined) {
      ElMessage.success(`${member.name} 积分已重置成功`)
    } else {
      ElMessage.success(`已重新邀请 ${member.name}，等待接受邀请`)
    }
    
    loadTeamMembers()
    loadPendingInvitations()
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    loading.value = false
  }
}

// 批量重置团队成员积分（排除自己）
async function batchResetMemberCredits() {
  const membersToReset = otherMembers.value
  
  if (membersToReset.length === 0) {
    ElMessage.warning('没有可重置的成员')
    return
  }
  
  try {
    await ElMessageBox.confirm(
      `确定要重置 ${membersToReset.length} 位成员的积分吗？此操作将移除并重新邀请这些成员。`,
      '批量重置积分',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
  } catch {
    return
  }
  
  batchResettingCredits.value = true
  let successCount = 0
  let failCount = 0
  
  try {
    for (const member of membersToReset) {
      try {
        // Step 1: 移除成员
        const removeResult = await invoke<any>('remove_team_member', {
          id: props.accountId,
          memberApiKey: member.api_key
        })
        
        if (!removeResult.success) {
          console.error(`移除成员 ${member.name} 失败:`, removeResult.error)
          failCount++
          continue
        }
        
        // Step 2: 重新邀请
        const inviteResult = await invoke<any>('invite_team_members', {
          id: props.accountId,
          users: [{ name: member.name, email: member.email }]
        })
        
        if (!inviteResult.success) {
          console.error(`邀请成员 ${member.name} 失败:`, inviteResult.error)
          failCount++
          continue
        }
        
        // Step 3: 自动接受邀请
        const autoJoinResults = await autoAcceptInvitations([member.email])
        const joined = autoJoinResults.some(r => r.success)
        
        if (joined) {
          successCount++
        } else {
          // 邀请已发送但未能自动加入（可能邮箱不在管理器中）
          successCount++
        }
      } catch (error) {
        console.error(`处理成员 ${member.name} 时出错:`, error)
        failCount++
      }
    }
    
    // 显示结果
    if (failCount === 0) {
      ElMessage.success(`成功重置 ${successCount} 位成员的积分`)
    } else {
      ElMessage.warning(`重置完成：成功 ${successCount} 位，失败 ${failCount} 位`)
    }
    
    // 刷新列表
    loadTeamMembers()
    loadPendingInvitations()
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    batchResettingCredits.value = false
  }
}

// 加载待处理邀请
async function loadPendingInvitations() {
  if (!props.accountId) return
  
  loading.value = true
  try {
    const result = await invoke<any>('get_pending_invitations', {
      id: props.accountId
    })
    
    if (result.success) {
      const data = result.data || {}
      console.log('[PendingInvitations] Full data:', JSON.stringify(data, null, 2))
      const preapprovals = data.subMesssage_1 || []
      console.log('[PendingInvitations] preapprovals:', preapprovals)
      
      if (Array.isArray(preapprovals)) {
        pendingInvitations.value = preapprovals.map((p: any) => ({
          id: p.string_1 || '',
          name: p.string_2 || '',
          email: p.string_3 || '',
          created_at: p.subMesssage_6?.int_1
        }))
      } else if (preapprovals && typeof preapprovals === 'object') {
        // 可能是单个对象而不是数组
        pendingInvitations.value = [{
          id: preapprovals.string_1 || '',
          name: preapprovals.string_2 || '',
          email: preapprovals.string_3 || '',
          created_at: preapprovals.subMesssage_6?.int_1
        }]
      } else {
        pendingInvitations.value = []
      }
      console.log('[PendingInvitations] Parsed:', pendingInvitations.value)
    } else {
      ElMessage.error(result.error || '获取待处理邀请失败')
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    loading.value = false
  }
}

// 撤销邀请
async function revokeInvitation(invitation: PendingInvitation) {
  loading.value = true
  try {
    const result = await invoke<any>('revoke_invitation', {
      id: props.accountId,
      approvalId: invitation.id
    })
    
    if (result.success) {
      ElMessage.success('邀请已撤销')
      loadPendingInvitations()
    } else {
      ElMessage.error(result.error || '撤销邀请失败')
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    loading.value = false
  }
}

// 加载我的邀请
async function loadMyInvitation() {
  if (!props.accountId) return
  
  loading.value = true
  try {
    const result = await invoke<any>('get_my_pending_invitation', {
      id: props.accountId
    })
    
    if (result.success && result.has_pending_invitation) {
      const data = result.data || {}
      myInvitation.value = {
        approval_id: data.subMesssage_1?.string_1 || '',
        team_name: data.string_3 || '',
        admin_name: data.string_2 || ''
      }
    } else {
      myInvitation.value = null
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    loading.value = false
  }
}

// 接受邀请
async function acceptInvitation() {
  if (!myInvitation.value) return
  
  loading.value = true
  try {
    const result = await invoke<any>('accept_invitation', {
      id: props.accountId,
      approvalId: myInvitation.value.approval_id
    })
    
    if (result.success) {
      ElMessage.success('已成功加入团队')
      myInvitation.value = null
    } else {
      ElMessage.error(result.error || '接受邀请失败')
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    loading.value = false
  }
}

// 拒绝邀请
async function rejectInvitation() {
  if (!myInvitation.value) return
  
  try {
    await ElMessageBox.confirm('确定要拒绝该邀请吗？', '确认', {
      type: 'warning'
    })
    
    loading.value = true
    const result = await invoke<any>('reject_invitation', {
      id: props.accountId,
      approvalId: myInvitation.value.approval_id
    })
    
    if (result.success) {
      ElMessage.success('已拒绝邀请')
      myInvitation.value = null
    } else {
      ElMessage.error(result.error || '拒绝邀请失败')
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error(error.toString())
    }
  } finally {
    loading.value = false
  }
}

// 邀请表单操作
function addInviteUser() {
  inviteForm.value.users.push({ name: '', email: '' })
}

function removeInviteUser(index: number) {
  inviteForm.value.users.splice(index, 1)
}

async function submitInvite() {
  const validUsers = inviteForm.value.users.filter(u => u.name && u.email)
  
  if (validUsers.length === 0) {
    ElMessage.warning('请至少填写一个有效的成员信息')
    return
  }
  
  inviting.value = true
  try {
    const result = await invoke<any>('invite_team_members', {
      id: props.accountId,
      users: validUsers
    })
    
    if (result.success) {
      const invitedCount = result.invited_count || validUsers.length
      
      // 如果开启了自动加入，尝试让管理器中的账号自动接受邀请
      if (autoJoinEnabled.value) {
        const autoJoinResults = await autoAcceptInvitations(validUsers.map(u => u.email))
        const joinedCount = autoJoinResults.filter(r => r.success).length
        
        if (joinedCount > 0) {
          ElMessage.success(`成功邀请 ${invitedCount} 位成员，${joinedCount} 位已自动加入`)
        } else {
          ElMessage.success(`成功邀请 ${invitedCount} 位成员`)
        }
      } else {
        ElMessage.success(`成功邀请 ${invitedCount} 位成员`)
      }
      
      showInviteDialog.value = false
      inviteForm.value.users = [{ name: '', email: '' }]
      loadPendingInvitations()
      loadTeamMembers()
    } else {
      ElMessage.error(result.error || '邀请失败')
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    inviting.value = false
  }
}

// 自动接受邀请（批量处理）
async function autoAcceptInvitations(emails: string[]): Promise<{ email: string; success: boolean }[]> {
  const results: { email: string; success: boolean }[] = []
  
  try {
    // 获取所有账号
    const accounts = await invoke<any[]>('get_all_accounts')
    
    for (const email of emails) {
      // 查找邮箱匹配的账号（忽略大小写）
      const matchedAccount = accounts.find((acc: any) => 
        acc.email?.toLowerCase() === email.toLowerCase()
      )
      
      if (matchedAccount) {
        console.log(`[AutoJoin] Found account for ${email}, attempting to accept invitation...`)
        
        try {
          // 使用该账号接受邀请
          const acceptResult = await invoke<any>('accept_invitation', {
            id: matchedAccount.id,
            approvalId: '' // 空字符串表示接受最新的邀请
          })
          
          results.push({ email, success: acceptResult.success === true })
          
          if (acceptResult.success) {
            console.log(`[AutoJoin] ${email} successfully joined the team`)
          }
        } catch (e) {
          console.error(`[AutoJoin] Failed to accept invitation for ${email}:`, e)
          results.push({ email, success: false })
        }
      } else {
        console.log(`[AutoJoin] No account found for ${email}`)
        results.push({ email, success: false })
      }
    }
  } catch (error) {
    console.error('[AutoJoin] Failed to get accounts:', error)
  }
  
  return results
}

// 格式化时间
function formatTime(timestamp?: number): string {
  if (!timestamp) return '-'
  const date = new Date(timestamp * 1000)
  return date.toLocaleString('zh-CN')
}

// 申请加入团队
async function submitJoinRequest() {
  if (!joinForm.value.inviteId.trim()) {
    ElMessage.warning('请输入邀请链接ID')
    return
  }
  
  joining.value = true
  try {
    const result = await invoke<any>('request_team_access', {
      id: props.accountId,
      inviteId: joinForm.value.inviteId.trim()
    })
    
    if (result.success) {
      ElMessage.success(result.message || '申请已提交，等待管理员审批')
      joinForm.value.inviteId = ''
    } else {
      ElMessage.error(result.error || '申请失败')
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    joining.value = false
  }
}

// 审批加入申请（管理员）
async function approveJoinRequest(member: TeamMember, action: 'approve' | 'reject') {
  loading.value = true
  try {
    const result = await invoke<any>('approve_team_join_request', {
      id: props.accountId,
      userApiKey: member.api_key,
      action: action
    })
    
    if (result.success) {
      ElMessage.success(result.message || (action === 'approve' ? '已同意加入' : '已拒绝加入'))
      loadTeamMembers()
    } else {
      ElMessage.error(result.error || '操作失败')
    }
  } catch (error: any) {
    ElMessage.error(error.toString())
  } finally {
    loading.value = false
  }
}

// 执行订阅转让
async function executeTransfer() {
  if (!transferForm.value.email.trim()) {
    ElMessage.warning('请输入目标用户邮箱')
    return
  }
  
  try {
    await ElMessageBox.confirm(
      `确定要将订阅转让给 ${transferForm.value.email} 吗？\n\n转让后您将被移出团队，此操作不可撤销！`,
      '确认转让',
      {
        confirmButtonText: '确认转让',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
  } catch {
    return
  }
  
  transferring.value = true
  transferStep.value = 0
  
  try {
    const targetEmail = transferForm.value.email.trim()
    const targetName = transferForm.value.name.trim() || targetEmail.split('@')[0]
    
    // Step 1: 禁用自己的访问权限
    transferStatus.value = '正在禁用自己的访问权限...'
    transferStep.value = 0
    
    // 找到当前账号的成员信息并禁用访问
    const currentMember = members.value.find(m => m.email?.toLowerCase() === currentAccountEmail.value?.toLowerCase())
    if (currentMember && !currentMember.disable_codeium) {
      await invoke<any>('update_codeium_access', {
        id: props.accountId,
        memberApiKey: currentMember.api_key,
        disableAccess: true
      })
    }
    
    // 查找目标用户是否已在团队中
    const existingMember = members.value.find(m => m.email?.toLowerCase() === targetEmail.toLowerCase())
    
    // Step 2: 邀请用户加入团队
    transferStatus.value = '正在邀请用户...'
    transferStep.value = 1
    
    let memberApiKey = existingMember?.api_key
    
    if (!existingMember) {
      // 用户不在团队中，发送邀请
      const inviteResult = await invoke<any>('invite_team_members', {
        id: props.accountId,
        users: [{ name: targetName, email: targetEmail }]
      })
      
      if (!inviteResult.success) {
        throw new Error(inviteResult.error || '邀请用户失败')
      }
      
      // 尝试自动接受邀请（如果邮箱在账号管理器中）
      const autoJoinResults = await autoAcceptInvitations([targetEmail])
      const joined = autoJoinResults.some(r => r.success)
      
      if (!joined) {
        // 用户未能自动加入，需要等待手动接受
        ElMessage.warning('邀请已发送，但用户需要手动接受邀请后才能完成转让。请在用户接受后重试。')
        showTransferDialog.value = false
        transferring.value = false
        loadPendingInvitations()
        return
      }
      
      // 重新加载成员列表获取新成员的 API Key
      await loadTeamMembers()
      const newMember = members.value.find(m => m.email?.toLowerCase() === targetEmail.toLowerCase())
      if (!newMember) {
        throw new Error('无法找到新加入的成员')
      }
      memberApiKey = newMember.api_key
    }
    
    // Step 3: 赋予用户管理员权限
    transferStatus.value = '正在授予管理员权限...'
    transferStep.value = 2
    
    await invoke<any>('add_user_role', {
      id: props.accountId,
      memberApiKey: memberApiKey,
      role: 'root.admin'
    })
    
    // Step 4: 把自己移除团队
    transferStatus.value = '正在移除自己...'
    transferStep.value = 3
    
    // 使用 Step 1 中找到的当前账号成员信息
    if (currentMember) {
      await invoke<any>('remove_team_member', {
        id: props.accountId,
        memberApiKey: currentMember.api_key
      })
    }
    
    transferStep.value = 4
    transferStatus.value = '转让完成！'
    
    ElMessage.success(`订阅已成功转让给 ${targetEmail}`)
    
    // 重置表单并关闭对话框
    showTransferDialog.value = false
    transferForm.value = { email: '', name: '' }
    dialogVisible.value = false
    
  } catch (error: any) {
    ElMessage.error(`转让失败: ${error.message || error}`)
  } finally {
    transferring.value = false
    transferStep.value = 0
    transferStatus.value = ''
  }
}

// 加载团队信息（获取邀请ID）
async function loadTeamInfo() {
  if (!props.accountId) return
  
  try {
    const result = await invoke<any>('get_current_user_parsed', {
      id: props.accountId
    })
    
    console.log('[TeamManagement] get_current_user_parsed result:', result)
    
    // 数据结构：result.data.team.invite_id, result.data.user.email
    if (result.success) {
      if (result.data?.team?.invite_id) {
        teamInviteId.value = result.data.team.invite_id
        console.log('[TeamManagement] Team invite ID:', teamInviteId.value)
      }
      if (result.data?.user?.email) {
        currentAccountEmail.value = result.data.user.email
        console.log('[TeamManagement] Current account email:', currentAccountEmail.value)
      }
    }
  } catch (error: any) {
    console.error('Failed to load team info:', error)
  }
}

// 复制邀请ID
async function copyInviteId() {
  if (!teamInviteId.value) return
  
  try {
    await navigator.clipboard.writeText(teamInviteId.value)
    ElMessage.success('邀请ID已复制到剪贴板')
  } catch (error) {
    // 备用方法
    const textarea = document.createElement('textarea')
    textarea.value = teamInviteId.value
    document.body.appendChild(textarea)
    textarea.select()
    document.execCommand('copy')
    document.body.removeChild(textarea)
    ElMessage.success('邀请ID已复制到剪贴板')
  }
}

// 复制完整邀请链接
async function copyInviteUrl() {
  if (!teamInviteId.value) return
  
  const inviteUrl = `https://windsurf.com/team/join/${teamInviteId.value}`
  
  try {
    await navigator.clipboard.writeText(inviteUrl)
    ElMessage.success('邀请链接已复制到剪贴板')
  } catch (error) {
    const textarea = document.createElement('textarea')
    textarea.value = inviteUrl
    document.body.appendChild(textarea)
    textarea.select()
    document.execCommand('copy')
    document.body.removeChild(textarea)
    ElMessage.success('邀请链接已复制到剪贴板')
  }
}

// 监听对话框打开
watch(dialogVisible, (val) => {
  if (val) {
    loadTeamInfo()
    loadTeamMembers()
    loadPendingInvitations()
    loadMyInvitation()
  }
})
</script>

<style scoped>
/* 全局容器 */
  .team-management-dialog :deep(.el-dialog__body) {
    padding: 0;
    background-color: #f8fafc;
  }

  .team-container {
    min-height: 550px;
    background: #f8fafc;
    display: flex;
    flex-direction: column;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  }

  /* 标签页美化 */
  :deep(.el-tabs--border-card) {
    border: none;
    box-shadow: none;
    background: transparent;
  }

  :deep(.el-tabs--border-card > .el-tabs__header) {
    background-color: #fff;
    border-bottom: 1px solid #f1f5f9;
    padding: 16px 32px 0;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.02);
  }

  :deep(.el-tabs--border-card > .el-tabs__header .el-tabs__item) {
    border: none;
    margin-right: 32px;
    padding: 12px 4px;
    height: auto;
    font-size: 15px;
    font-weight: 500;
    color: #64748b;
    transition: all 0.2s ease;
  }

  :deep(.el-tabs--border-card > .el-tabs__header .el-tabs__item:hover) {
    color: #3b82f6;
  }

  :deep(.el-tabs--border-card > .el-tabs__header .el-tabs__item.is-active) {
    color: #3b82f6;
    font-weight: 600;
    border-bottom: 3px solid #3b82f6;
    background: transparent;
  }

  :deep(.el-tabs--border-card > .el-tabs__content) {
    padding: 24px 32px;
  }

  /* 邀请链接卡片 - 视觉焦点 */
  .invite-link-section {
    background: linear-gradient(120deg, #4f46e5 0%, #7c3aed 100%);
    border-radius: 16px;
    padding: 24px;
    margin-bottom: 32px;
    color: white;
    box-shadow: 0 10px 25px -5px rgba(79, 70, 229, 0.3);
    position: relative;
    overflow: hidden;
  }

  .invite-link-section::before {
    content: '';
    position: absolute;
    top: 0;
    right: 0;
    width: 200px;
    height: 200px;
    background: radial-gradient(circle, rgba(255,255,255,0.1) 0%, transparent 70%);
    border-radius: 50%;
    transform: translate(30%, -30%);
  }

  .invite-link-label {
    display: flex;
    align-items: center;
    gap: 8px;
    color: rgba(255, 255, 255, 0.9);
    font-size: 14px;
    font-weight: 500;
    margin-bottom: 12px;
  }

  .invite-link-label .el-icon {
    color: white;
  }

  .invite-link-content {
    display: flex;
    align-items: center;
    gap: 12px;
    background: rgba(255, 255, 255, 0.1);
    padding: 6px 6px 6px 16px;
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(10px);
  }

  .invite-id-input {
    flex: 1;
  }

  .invite-id-input :deep(.el-input__wrapper) {
    box-shadow: none !important;
    background: transparent;
    padding: 0;
  }

  .invite-id-input :deep(.el-input__inner) {
    font-family: 'JetBrains Mono', 'Monaco', monospace;
    font-size: 15px;
    color: white;
    letter-spacing: 0.5px;
  }

  .invite-link-content .el-button {
    background: white;
    border: none;
    color: #4f46e5;
    font-weight: 600;
    height: 32px;
    padding: 0 16px;
    border-radius: 8px;
    transition: all 0.2s;
  }

  .invite-link-content .el-button:hover {
    background: #f0fdf4;
    transform: translateY(-1px);
  }

  /* 操作栏 */
  .tab-header {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-bottom: 20px;
  }

  .tab-header .el-button {
    border-radius: 8px;
    padding: 9px 16px;
    height: auto;
  }

  /* 表格样式优化 */
  .member-table {
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05), 0 2px 4px -1px rgba(0, 0, 0, 0.03);
    border: 1px solid #f1f5f9;
    background: white;
  }

  :deep(.el-table th.el-table__cell) {
    background-color: #f8fafc;
    color: #475569;
    font-weight: 600;
    font-size: 13px;
    height: 54px;
    border-bottom: 1px solid #e2e8f0;
  }

  :deep(.el-table td.el-table__cell) {
    padding: 16px 0;
    border-bottom: 1px solid #f1f5f9;
  }

  :deep(.el-table--enable-row-hover .el-table__body tr:hover > td.el-table__cell) {
    background-color: #f8fafc;
  }

  .member-cell-name {
    font-size: 15px;
    font-weight: 600;
    color: #1e293b;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .member-cell-email {
    font-size: 13px;
    color: #64748b;
    margin-top: 4px;
  }

  .role-tag {
    font-size: 11px;
    padding: 2px 8px;
    height: 22px;
    line-height: 18px;
    border-radius: 6px;
    font-weight: 600;
    border: none;
    background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
    color: white;
  }

  .time-text {
    font-size: 13px;
    color: #64748b;
    background: #f1f5f9;
    padding: 4px 8px;
    border-radius: 6px;
  }

  /* 状态徽章 */
  :deep(.el-tag--success) {
    background-color: #dcfce7;
    border-color: transparent;
    color: #166534;
  }

  :deep(.el-tag--danger) {
    background-color: #fee2e2;
    border-color: transparent;
    color: #991b1b;
  }

  /* 按钮样式 */
  .el-button--text {
    font-weight: 500;
  }

  .el-button--info.is-text {
    color: #64748b;
  }
  .el-button--info.is-text:hover {
    color: #3b82f6;
    background: #eff6ff;
  }

  .el-button--danger.is-text {
    color: #ef4444;
  }
  .el-button--danger.is-text:hover {
    background: #fef2f2;
  }

  /* 空状态 */
  .empty-state {
    padding: 64px 0;
    text-align: center;
    background: white;
    border-radius: 16px;
    border: 1px solid #f1f5f9;
  }

  /* 成员详情卡片 */
  .info-card {
    background: white;
    border: 1px solid #f1f5f9;
    border-radius: 16px;
    padding: 24px;
    margin-bottom: 24px;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .member-avatar {
    background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
    color: white;
    font-size: 20px;
    border: 4px solid #eff6ff;
  }

  .member-name {
    font-size: 18px;
    font-weight: 700;
    color: #1e293b;
  }

  /* 详情表单 */
  .detail-form .el-form-item {
    margin-bottom: 24px;
  }

  .detail-form :deep(.el-form-item__label) {
    color: #64748b;
    font-weight: 500;
  }

  /* 邀请卡片优化 */
  .invitation-card {
    background: linear-gradient(135deg, #4f46e5 0%, #06b6d4 100%);
    color: white;
    padding: 40px;
    border-radius: 20px;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
    max-width: 480px;
    margin: 20px auto;
    position: relative;
    overflow: hidden;
  }

  .invitation-card::after {
    content: '';
    position: absolute;
    top: -50px;
    right: -50px;
    width: 150px;
    height: 150px;
    background: rgba(255,255,255,0.1);
    border-radius: 50%;
  }

  .invitation-actions .el-button {
    border: none;
    height: 40px;
    font-size: 15px;
    border-radius: 10px;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  }

  /* 邀请成员对话框 */
  .invite-user-row {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 16px;
    background: #f8fafc;
    border-radius: 12px;
    margin-bottom: 12px;
    border: 1px solid #e2e8f0;
  }

  .invite-user-fields {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .invite-user-fields .el-form-item {
    margin-bottom: 0;
  }

  .invite-user-row .delete-btn {
    margin-top: 8px;
    flex-shrink: 0;
  }

  .add-more-btn {
    width: 100%;
    height: 44px;
    border: 2px dashed #cbd5e1;
    background: transparent;
    color: #64748b;
    font-weight: 500;
    border-radius: 10px;
    transition: all 0.2s;
  }

  .add-more-btn:hover {
    border-color: #3b82f6;
    color: #3b82f6;
    background: #eff6ff;
  }

  .auto-join-section {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 20px;
    padding: 16px;
    background: linear-gradient(135deg, #ecfdf5 0%, #d1fae5 100%);
    border-radius: 10px;
    border: 1px solid #a7f3d0;
  }

  .auto-join-label {
    font-size: 14px;
    color: #065f46;
    font-weight: 600;
  }

  .help-icon {
    color: #10b981;
    cursor: help;
  }

  /* 详情对话框底部 */
  .dialog-footer {
    display: flex;
    justify-content: space-between;
    width: 100%;
    gap: 12px;
  }

  .form-tip {
    font-size: 12px;
    color: #64748b;
    margin-top: 8px;
    line-height: 1.5;
  }

  .info-value {
    color: #1e293b;
    font-size: 14px;
    font-weight: 500;
  }

  .member-email {
    font-size: 14px;
    color: #64748b;
  }

  .member-info {
    flex: 1;
  }

  .info-header {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  /* 转让进度样式 */
  .transfer-progress {
    margin-top: 20px;
    padding: 16px;
    background: #f8fafc;
    border-radius: 8px;
  }

  .transfer-status {
    margin-top: 12px;
    text-align: center;
    color: #64748b;
    font-size: 14px;
  }
</style>
