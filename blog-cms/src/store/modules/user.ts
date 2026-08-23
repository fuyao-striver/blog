import { defineStore } from "pinia";
import type { UserState } from "../interface";
import type { LoginForm } from "@/api/login/type";
import { setToken } from "@/utils/token";
import { login } from "@/api/login";
import { getUserInfo } from "@/api/user";

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
    GetInfo() {
      return new Promise((resolve, reject) => {
        getUserInfo()
          .then(({ data }) => {
            if (data.flag) {
              this.id = data.data.id;
              this.avatar = data.data.avatar;
              ((this.roleList = data.data.roleList), (this.permissionList = data.data.permissionList));
            }
            resolve(data);
          })
          .catch((error) => {
            reject(error);
          });
      });
    },
  },
});

export default useUserStore;
