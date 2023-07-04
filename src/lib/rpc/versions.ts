import { toastStore } from "$lib/stores/ToastStore";
import { invoke, type InvokeArgs } from "@tauri-apps/api/tauri";
import { errorLog, exceptionLog } from "./logging";

export type VersionFolders = null | "official" | "unofficial" | "devel";

export async function listDownloadedVersions(
  folder: VersionFolders
): Promise<string[]> {
  return await invoke_and_log(
    "list_downloaded_versions",
    () => [],
    { versionFolder: folder }
  );
}

export async function downloadOfficialVersion(
  version: String,
  url: String
): Promise<boolean> {
  let success = await invoke_and_log(
    "download_version",
    () => false,
    {
      version: version,
      versionFolder: "official",
      url: url,
    }
  );

  return success !== false;
}

export async function removeVersion(
  version: String,
  versionFolder: String
): Promise<boolean> {
  let success = await invoke_and_log(
    "remove_version",
    () => false,
    {
      version: version,
      versionFolder: versionFolder,
    }
  );

  return success !== false;
}

export async function openVersionFolder(folder: VersionFolders) {
  return await invoke_and_log(
    "open_version_folder",
    () => undefined,
    { versionFolder: folder }
  );
}

export async function getActiveVersion(): Promise<string | null> {
  return await invoke_and_log(
    "get_active_tooling_version",
    () => null,
  );
}

export async function getActiveVersionFolder(): Promise<VersionFolders> {
  return await invoke_and_log(
    "get_active_tooling_version_folder",
    () => null,
  );
}

export async function ensureActiveVersionStillExists(): Promise<boolean> {
  return await invoke_and_log(
    "ensure_active_version_still_exists",
    () => false,
  );
}

async function invoke_and_log<T>(
  method: string,
  handle: (e: any) => T,
  args?: InvokeArgs,
): Promise<T> {
  try {
    return await invoke(method, args);
  } catch (e) {
    if (e.name && e.message) {
      toastStore.makeToast(e.message, "error");
      errorLog(`Error invoking ${method}: (${e.name}) ${e.message}`);
    } else if (typeof e === "string") {
      toastStore.makeToast(e, "error");
      errorLog(`Error invoking ${method}: ${e}`);
    } else {
      exceptionLog(`Error invoking ${method}`, e);
    }
    return handle(e);
  }
}
