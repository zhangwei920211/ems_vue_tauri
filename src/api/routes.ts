import { invoke } from "@tauri-apps/api/core";

type Result = {
  success: boolean;
  data: Array<any>;
};

export const getAsyncRoutes = async (): Promise<Result> => {
  try {
    const data = await invoke<Array<any>>("get_async_routes");
    return {
      success: true,
      data
    };
  } catch (error) {
    console.error("获取路由失败:", error);
    return {
      success: false,
      data: []
    };
  }
};
