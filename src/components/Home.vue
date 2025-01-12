<template>
  <h2 id="result" class="result">Welcome to GitProxy</h2>
  <div class="mb-4">
    <el-button type="info" round @click="switchProxy('default')">Default</el-button>
    <el-button type="primary" round @click="switchProxy('http')">HTTP</el-button>
    <el-button type="success" round @click="switchProxy('socks')">Socks</el-button>
    <el-button round @click="openSettings">Settings</el-button>

    <!-- Settings dialog -->
    <el-dialog v-model="settingsVisible" title="Settings" width="70%" center>
      <el-form :model="form" label-width="120px">
        <el-form-item label="HTTP">
          <el-input
            v-model="form.httpAddr"
            placeholder="127.0.0.1:7890"
          ></el-input>
        </el-form-item>
        <el-form-item label="Socks5">
          <el-input
            v-model="form.socks5Addr"
            placeholder="127.0.0.1:80"
          ></el-input>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="resetSettings">Reset</el-button>
          <el-button type="primary" @click="saveSettings">Save</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- Copyright information -->
    <el-footer class="footer">
      <p style="color: black">&copy; {{ currentYear }} YangYezhuang.</p>
    </el-footer>
  </div>
</template>

<script setup>
import { computed, reactive, ref } from "vue";
import { ElMessage } from "element-plus";
import { invoke } from "@tauri-apps/api/core";

const switchState = ref("");
const settingsVisible = ref(false);
const form = reactive({
  httpAddr: "",
  socks5Addr: "",
});
const currentYear = computed(() => new Date().getFullYear());

async function switchProxy(mode) {
  switchState.value = await invoke("switch_proxy", { mode: mode });
  ElMessage({
    message: `Use ${mode} Mode`,
    type: "success",
  });
}

function _switchProxy(mode) {
  let addr;
  const item = localStorage.getItem("config");
  if (item != null) {
    const parse = JSON.parse(item);
    switch (mode) {
      case "default":
        invoke("switch_proxy", { mode, addr }).then((result) => {
           ElMessage({
            message: `Use ${mode} Mode`,
            type: "success",
          });
        });
        break;
      case "http":
        addr = parse.httpAddr;
        if (addr == null || addr === "") {
          ElMessage({
            message: `Please setting HTTP addr`,
            type: "success",
          });
        } else {
          invoke("switch_proxy", { mode: mode,addr: addr }).then((result) => {
            ElMessage({
              message: `Use ${mode} Mode`,
              type: "success",
            });
          });
        }
        break;
      case "socks":
        addr = parse.socks5Addr;
        if (addr == null || addr === "") {
          ElMessage({
            message: `Please setting Socks addr`,
            type: "success",
          });
        } else {
          invoke("switch_proxy", { mode: mode, addr: addr }).then((result) => {
            ElMessage({
              message: `Use ${mode} Mode`,
              type: "success",
            });
          });
        }
        break;
      default:
    }
  }
}

const openSettings = async () => {
  const item = localStorage.getItem("config");
  if (item == null) {
    localStorage.setItem("config", null);
  } else {
    const parse = JSON.parse(item);
    // form.httpAddr = parse.httpAddr;
    // form.socks5Addr = parse.socks5Addr;
  }
  settingsVisible.value = true;
};

const saveSettings = async () => {
  localStorage.setItem("config", JSON.stringify(form));
  ElMessage({
    message: "Saved successfully",
    type: "success",
  });
  settingsVisible.value = false;
};

const resetSettings = async () => {
  localStorage.clear();
  ElMessage({
    message: "Reset successfully",
    type: "success",
  });
  settingsVisible.value = false;
};
</script>

<style scoped>
.result {
  height: 20px;
  line-height: 20px;
  margin: 1.5rem auto;
  color: #000;
}
.footer {
  background-color: #fff;
  width: 100%;
  text-align: center;
  padding: 16px 0;
  color: #4a5568;
  position: fixed;
  bottom: 0;
  left: 0;
}
</style>
