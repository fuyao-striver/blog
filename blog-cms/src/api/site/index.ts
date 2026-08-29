import type { Result } from "@/modal";
import type { AxiosPromise } from "axios";
import type { SiteConfig } from "./types";
import requests from "@/utils/request";

/**
 * 查看网站配置
 * @returns 网站配置
 */
export const getSiteConfig = (): AxiosPromise<Result<SiteConfig>> => {
  return requests({
    url: "/admin/site/list",
    method:"GET"
   })
 }