import { invoke } from "@tauri-apps/api/core";

export type UserResult = {
  success: boolean;
  data: {
    /** 头像 */
    avatar: string;
    /** 用户名 */
    username: string;
    /** 昵称 */
    nickname: string;
    /** 当前登录用户的角色 */
    roles: Array<string>;
    /** 按钮级别权限 */
    permissions: Array<string>;
    /** `token` */
    accessToken: string;
    /** 用于调用刷新`accessToken`的接口时所需的`token` */
    refreshToken: string;
    /** `accessToken`的过期时间（格式'xxxx/xx/xx xx:xx:xx'） */
    expires: Date;
  };
};

export type RefreshTokenResult = {
  success: boolean;
  data: {
    /** `token` */
    accessToken: string;
    /** 用于调用刷新`accessToken`的接口时所需的`token` */
    refreshToken: string;
    /** `accessToken`的过期时间（格式'xxxx/xx/xx xx:xx:xx'） */
    expires: Date;
  };
};

/** 登录 */
export const getLogin = async (data?: {
  username: string;
  password: string;
}): Promise<UserResult> => {
  try {
    const response = await invoke<any>("login", { request: data });
    if (response.success && response.data) {
      // 转换后端返回的数据格式以匹配前端期望的格式
      return {
        success: true,
        data: {
          avatar: response.data.avatar,
          username: response.data.username,
          nickname: response.data.nickname,
          roles: response.data.roles,
          permissions: response.data.permissions,
          accessToken: response.data.access_token,
          refreshToken: response.data.refresh_token,
          expires: new Date(response.data.expires)
        }
      };
    }
    return {
      success: false,
      data: null
    };
  } catch (error) {
    console.error("登录失败:", error);
    return {
      success: false,
      data: null
    };
  }
};

/** 刷新`token` */
export const refreshTokenApi = async (data?: {
  refreshToken: string;
}): Promise<RefreshTokenResult> => {
  try {
    const response = await invoke<any>("refresh_token", {
      request: { refresh_token: data.refreshToken }
    });
    if (response.success && response.data) {
      // 转换后端返回的数据格式以匹配前端期望的格式
      return {
        success: true,
        data: {
          accessToken: response.data.access_token,
          refreshToken: response.data.refresh_token,
          expires: new Date(response.data.expires)
        }
      };
    }
    return {
      success: false,
      data: null
    };
  } catch (error) {
    console.error("刷新token失败:", error);
    return {
      success: false,
      data: null
    };
  }
};
