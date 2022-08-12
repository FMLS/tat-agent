use serde::{Deserialize, Serialize};
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WsMsg<T> {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub seq: u64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyStart {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub user_name: String,
    #[serde(default)]
    pub cols: u16,
    #[serde(default)]
    pub rows: u16,
    #[serde(default)]
    pub init_block: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyInput {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub input: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyResize {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub cols: u16,
    #[serde(default)]
    pub rows: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyStop {
    #[serde(default)]
    pub session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyOutput {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub output: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyReady {
    #[serde(default)]
    pub session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyError {
    #[serde(default)]
    pub session_id: String,
    pub reason: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyBinBase<T> {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub custom_data: String,
    #[serde(default)]
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PtyBinErrResp {
    pub error: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFileReq {
    pub path: String,
    pub mode: u32,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFileResp {
    pub created: bool,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteFileReq {
    #[serde(default)]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteFileResp {
    pub deleted: bool,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ReadFileReq {
    pub path: String,
    #[serde(default = "default_read_offset")]
    pub offset: usize,
    #[serde(default = "default_read_size")]
    pub size: usize,
}

fn default_read_size() -> usize {
    return usize::MAX;
}

fn default_read_offset() -> usize {
    return 0;
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ReadFileResp {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub offset: usize,
    pub length: usize,
    pub is_last: bool,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WriteFileReq {
    pub path: String,
    #[serde(default = "default_write_offset")]
    pub offset: usize,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
fn default_write_offset() -> usize {
    return usize::MAX;
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WriteFileResp {
    pub length: usize,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FileExistsReq {
    #[serde(default)]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FileExistResp {
    pub exists: bool,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FileInfoReq {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub custom_data: String,
    #[serde(default)]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FileInfoResp {
    pub r#type: String,
    pub name: String,
    pub size: u64,
    pub modify_time: u64,
    pub access_time: u64,
    #[cfg(unix)]
    pub owner: u32,
    #[cfg(unix)]
    pub group: u32,
    #[cfg(unix)]
    pub rights: String,
    #[cfg(unix)]
    pub longname: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ListPathReq {
    pub path: String,
    #[serde(default = "default_list_filter")]
    pub filter: String,
}

fn default_list_filter() -> String {
    return "*".to_string();
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ListPathResp {
    pub index: u32,
    pub is_last: bool,
    pub files: Vec<FileInfoResp>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ExecCmdReq {
    pub cmd: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ExecCmdResp {
    pub output: String,
}
