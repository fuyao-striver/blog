import { defineStore } from "pinia";
import type { UserState } from "@/stores/interface";
import { login, logout } from "@/api/login";
import { removeToken, setToken } from "@/utils/token.ts";
import type { LoginForm } from "@/api/login/types";

const useUserStore = defineStore("useUserStore", {
  state: (): UserState => ({
    id: null,
    avatar: "",
    roleList: [],
    permissionList: [],
  }),
  actions: {
    LogIn(LoginForm: LoginForm) {
      return new Promise((resolve, reject) => {
        login(LoginForm)
          .then(({ data }) => {
            if (data.flag) {
              setToken(data.data);
              resolve(data);
            } else {
              reject(data.msg);
            }
          })
          .catch((error) => {
            reject(error);
          });
      });
    },
    LogOut() {
      return new Promise((resolve, reject) => {
        logout()
          .then(() => {
            this.id = null;
            this.avatar = "";
            this.roleList = [];
            this.permissionList = [];
            removeToken();
            resolve(null);
          })
          .catch((error) => {
            reject(error);
          });
      });
    },
  },
});

export default useUserStore;
