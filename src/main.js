const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.opener;

// ===== DOM Elements =====
const deviceListEl = document.getElementById("device-list");
const deviceCountEl = document.getElementById("device-count");
const adbStatusEl = document.getElementById("adb-status");
const adbStatusTextEl = document.getElementById("adb-status-text");
const logContentEl = document.getElementById("log-content");
const refreshBtn = document.getElementById("refresh-btn");
const settingsPanelEl = document.getElementById("settings-panel");
const adbPathInputEl = document.getElementById("adb-path-input");
const adbPathStatusEl = document.getElementById("adb-path-status");

// ===== Logging =====
function addLog(message, type = "info") {
  const entry = document.createElement("div");
  entry.className = `log-entry log-${type}`;
  const time = new Date().toLocaleTimeString("en-US", {
    hour12: false,
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
  entry.textContent = `[${time}] ${message}`;
  logContentEl.appendChild(entry);
  logContentEl.scrollTop = logContentEl.scrollHeight;
}

window.clearLog = function () {
  logContentEl.innerHTML = "";
  addLog("Log cleared.");
};

// ===== ADB Status Check =====
async function checkAdbStatus() {
  try {
    const result = await invoke("get_adb_status");
    if (result.success) {
      adbStatusEl.className = "status-badge status-connected";
      adbStatusTextEl.textContent = "ADB Ready";
      addLog(result.message, "success");
    } else {
      adbStatusEl.className = "status-badge status-error";
      adbStatusTextEl.textContent = "ADB Not Found";
      addLog(result.message, "error");
    }
  } catch (e) {
    adbStatusEl.className = "status-badge status-error";
    adbStatusTextEl.textContent = "ADB Error";
    addLog(`ADB check failed: ${e}`, "error");
  }
}

// ===== Device List =====
function createDeviceCard(device) {
  const card = document.createElement("div");
  card.className = "device-card";

  const isWireless = device.device_type === "wireless";
  const iconSvg = isWireless
    ? `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M5 12.55a11 11 0 0 1 14.08 0"/>
        <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
        <circle cx="12" cy="20" r="1"/>
       </svg>`
    : `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="5" y="2" width="14" height="20" rx="2" ry="2"/>
        <line x1="12" y1="18" x2="12.01" y2="18"/>
       </svg>`;

  card.innerHTML = `
    <div class="device-info">
      <div class="device-icon ${device.device_type}">
        ${iconSvg}
      </div>
      <div>
        <div class="device-serial">${escapeHtml(device.serial)}</div>
        <div class="device-status">
          <span class="type-badge ${device.device_type}">${device.device_type}</span>
          ${escapeHtml(device.status)}
        </div>
      </div>
    </div>
    <div class="device-actions">
      ${
        isWireless
          ? `<button class="btn btn-danger btn-sm" onclick="disconnectDevice('${escapeHtml(device.serial)}')">
               <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                 <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
               </svg>
               Disconnect
             </button>`
          : `<button class="btn btn-success btn-sm" onclick="enableWireless('${escapeHtml(device.serial)}', this)">
               <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                 <path d="M5 12.55a11 11 0 0 1 14.08 0"/>
                 <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
                 <circle cx="12" cy="20" r="1"/>
               </svg>
               Enable Wireless
             </button>`
      }
    </div>
  `;

  return card;
}

function renderDevices(devices) {
  deviceListEl.innerHTML = "";

  if (devices.length === 0) {
    deviceListEl.innerHTML = `
      <div class="empty-state">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="empty-icon">
          <rect x="5" y="2" width="14" height="20" rx="2" ry="2"/>
          <line x1="12" y1="18" x2="12.01" y2="18"/>
        </svg>
        <p>No devices detected</p>
        <p class="empty-hint">Connect an Android device via USB with USB Debugging enabled</p>
      </div>
    `;
    deviceCountEl.textContent = "0 devices";
    return;
  }

  deviceCountEl.textContent = `${devices.length} device${devices.length !== 1 ? "s" : ""}`;
  devices.forEach((device) => {
    deviceListEl.appendChild(createDeviceCard(device));
  });
}

window.refreshDevices = async function () {
  refreshBtn.disabled = true;
  refreshBtn.innerHTML = `<div class="spinner"></div> Scanning...`;
  addLog("Scanning for devices...");

  try {
    const devices = await invoke("list_devices");
    renderDevices(devices);
    addLog(`Found ${devices.length} device(s).`, devices.length > 0 ? "success" : "info");
  } catch (e) {
    addLog(`Error listing devices: ${e}`, "error");
    renderDevices([]);
  } finally {
    refreshBtn.disabled = false;
    refreshBtn.innerHTML = `
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="23 4 23 10 17 10"/>
        <polyline points="1 20 1 14 7 14"/>
        <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
      </svg>
      Refresh Devices
    `;
  }
};

// ===== Enable Wireless =====
window.enableWireless = async function (serial, btnEl) {
  btnEl.disabled = true;
  btnEl.innerHTML = `<div class="spinner"></div> Connecting...`;
  addLog(`Enabling wireless debugging for ${serial}...`, "warning");

  try {
    const result = await invoke("enable_wireless", { serial: serial });
    if (result.success) {
      addLog(result.message, "success");
    } else {
      addLog(result.message, "error");
    }
    // Refresh device list after a short delay
    setTimeout(() => window.refreshDevices(), 1000);
  } catch (e) {
    addLog(`Error: ${e}`, "error");
    btnEl.disabled = false;
    btnEl.innerHTML = `Enable Wireless`;
  }
};

// ===== Disconnect =====
window.disconnectDevice = async function (address) {
  addLog(`Disconnecting ${address}...`, "warning");

  try {
    const result = await invoke("disconnect_device", { address: address });
    if (result.success) {
      addLog(result.message, "success");
    } else {
      addLog(result.message, "error");
    }
    setTimeout(() => window.refreshDevices(), 500);
  } catch (e) {
    addLog(`Error: ${e}`, "error");
  }
};

// ===== Utility =====
function escapeHtml(text) {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}

// ===== Settings =====
window.toggleSettings = function () {
  settingsPanelEl.classList.toggle("hidden");
  if (!settingsPanelEl.classList.contains("hidden")) {
    loadAdbConfig();
  }
};

async function loadAdbConfig() {
  try {
    const config = await invoke("get_adb_config");
    adbPathInputEl.value = config.path === "adb" ? "" : config.path;
  } catch (e) {
    addLog(`Failed to load ADB config: ${e}`, "error");
  }
}

window.saveAdbPath = async function () {
  const path = adbPathInputEl.value.trim();
  adbPathStatusEl.textContent = "Verifying...";
  adbPathStatusEl.className = "adb-path-status";

  try {
    const result = await invoke("set_adb_path", { config: { path: path || "adb" } });
    if (result.success) {
      adbPathStatusEl.textContent = "✓ " + result.message;
      adbPathStatusEl.className = "adb-path-status success";
      addLog("ADB path updated", "success");
      // Refresh ADB status and device list
      await checkAdbStatus();
      await window.refreshDevices();
      setTimeout(() => {
        settingsPanelEl.classList.add("hidden");
        adbPathStatusEl.textContent = "";
      }, 1000);
    } else {
      adbPathStatusEl.textContent = "✗ " + result.message;
      adbPathStatusEl.className = "adb-path-status error";
    }
  } catch (e) {
    adbPathStatusEl.textContent = "✗ Error: " + e;
    adbPathStatusEl.className = "adb-path-status error";
  }
};

window.resetAdbPath = async function () {
  adbPathInputEl.value = "";
  await window.saveAdbPath();
};

window.browseAdbPath = async function () {
  // Open a file dialog using the opener plugin
  // Note: Full file dialog requires additional Tauri plugin
  addLog("Please manually enter the ADB path. Common locations:", "info");
  addLog("  - /opt/homebrew/bin/adb (Homebrew on Apple Silicon)", "info");
  addLog("  - /usr/local/bin/adb (Homebrew on Intel Mac)", "info");
  addLog("  - ~/Library/Android/sdk/platform-tools/adb (Android Studio)", "info");
};

// ===== Init =====
document.addEventListener("DOMContentLoaded", async () => {
  await checkAdbStatus();
  await window.refreshDevices();
});
