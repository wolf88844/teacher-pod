use serde::{Deserialize, Serialize};

/// 表示一个播客的结构体
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Podcast {
    /// 播客的唯一标识符
    pub id: String,
    /// 播客的类型
    pub r#type: String,
    /// 播客的图片链接
    pub image: String,
    /// 播客的标题
    pub title: String,
    /// 播客的所属国家
    pub country: String,
    /// 播客的语言
    pub language: String,
    /// 播客的发布者
    pub publisher: String,
    /// 播客的缩略图链接
    pub thumbnail: String,
    /// 播客是否已被声明
    pub is_claimed: bool,
    /// 播客的描述
    pub description: String,
    /// 播客的总集数
    pub total_episodes: i32,
    /// 播客音频的总时长（秒）
    pub audio_length_sec: i32,
    /// 播客是否包含明确内容
    pub explicit_content: bool,
    /// 最新一集的标识符
    pub latest_episode_id: String,
    /// 播客在全球收听评分中的排名
    pub listen_score_global_rank: String,
    /// 播客的剧集列表，默认为空
    #[serde(default = "default_episodes")]
    pub episodes: Vec<Episode>,
}

/// 创建一个默认的空剧集列表
fn default_episodes() -> Vec<Episode> {
    vec![]
}

/// 表示一个播客剧集的结构体
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Episode {
    /// 剧集的唯一标识符
    pub id: String,
    /// 剧集的链接
    pub link: String,
    /// 剧集的音频链接
    pub audio: String,
    /// 剧集的图片链接
    pub image: String,
    /// 剧集的标题，默认为空字符串
    #[serde(default)]
    pub title: String,
    /// 剧集的缩略图链接
    pub thumbnail: String,
    /// 剧集的描述，默认为空字符串
    #[serde(default)]
    pub description: String,
    /// 剧集的原始标题，默认为空字符串
    #[serde(default)]
    pub title_original: String,
    /// 剧集的高亮标题，默认为空字符串
    #[serde(default)]
    pub title_highlighted: String,
    /// 剧集的原始描述，默认为空字符串
    #[serde(default)]
    pub description_original: i32,
    /// 剧集的高亮描述，默认为空字符串
    #[serde(default)]
    pub description_highlighted: i32,
    /// 剧集音频的时长（秒）
    pub audio_length_sec: u32,
}

/// 表示最佳播客列表的结构体
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct BestPodcasts {
    /// 列表的唯一标识符
    pub id: i32,
    /// 列表的名称
    pub name: String,
    /// 列表中的播客总数
    pub total: i32,
    /// 是否有下一页
    pub has_next: bool,
    /// 列表中的播客列表
    pub podcasts: Vec<Podcast>,
    /// 父列表的标识符
    pub parent_id: i32,
    /// 当前页码
    pub page_number: i32,
    /// 是否有上一页
    pub has_previous: bool,
    /// 下一页的页码
    pub next_page_number: i32,
    /// 上一页的页码
    pub previous_page_number: i32,
}
