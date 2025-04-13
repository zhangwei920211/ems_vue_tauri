// 删除未使用的导入
// import { isClient } from "@pureadmin/utils";

// 强制声明window._TAURI_类型，防止TS类型错误
declare global {
  interface Window {
    __TAURI__?: any;
    __TAURI_METADATA__?: any;
    _TAURI_IPC_?: any;
  }
}

// 检查是否在 Tauri 环境中运行
export const isTauri = (): boolean => {
  if (typeof window === 'undefined') {
    return false;
  }
  
  return typeof window.__TAURI__ !== 'undefined';
};

// 获取 Tauri 版本信息
export const getTauriVersion = async (): Promise<string | null> => {
  if (!isTauri()) return null;
  
  try {
    try {
      const { getVersion } = await import("@tauri-apps/api/app");
      const version = await getVersion();
      return version;
    } catch (_) {
      // 尝试直接从window.__TAURI__对象获取
      if (window.__TAURI_METADATA__?.version) {
        return window.__TAURI_METADATA__.version;
      }
      return "未知版本";
    }
  } catch (_) {
    return null;
  }
};

// 获取系统信息
export const getSystemInfo = async (): Promise<any | null> => {
  if (!isTauri()) return null;
  
  try {
    try {
      // 使用通用API导入方式
      const api = await import("@tauri-apps/api");
      // 检查api对象中是否有os功能
      if (api && typeof api.os?.type === 'function') {
        const info = {
          os: await api.os.type(),
          arch: await api.os.arch()
        };
        return info;
      }
      throw new Error("API对象中没有os功能");
    } catch (_) {
      // 如果标准API失败，返回默认值
      return { os: "未知操作系统", arch: "未知架构" };
    }
  } catch (_) {
    return null;
  }
};
