use serde::Deserialize;

pub struct IniEntry {
    pub key: &'static str,
    pub value: String,
}

pub struct SectionPatch {
    pub section: &'static str,
    pub entries: Vec<IniEntry>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SegatoolsPatch {
    pub dns: Option<DnsPatch>,
    pub keychip: Option<KeychipPatch>,
    pub vfs: Option<VfsPatch>,
    pub gpio: Option<GpioPatch>,
    pub gfx: Option<GfxPatch>,
    pub aime: Option<AimePatch>,
    pub io3: Option<Io3Patch>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsPatch {
    pub default: String,
    pub aimedb: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeychipPatch {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VfsPatch {
    pub option: String,
    pub amfs: String,
    pub appdata: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpioPatch {
    pub dipsw1: u8,
    pub dipsw2: u8,
    pub dipsw3: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GfxPatch {
    pub windowed: u8,
    pub framed: u8,
    pub monitor: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AimePatch {
    pub enable: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Io3Patch {
    pub test: String,
    pub service: String,
    pub coin: String,
}

impl SegatoolsPatch {
    pub fn sections(&self) -> Vec<SectionPatch> {
        let mut patches: Vec<SectionPatch> = Vec::new();

        if let Some(dns) = &self.dns {
            let mut entries = vec![IniEntry {
                key: "default",
                value: dns.default.clone(),
            }];
            if let Some(aimedb) = &dns.aimedb {
                entries.push(IniEntry {
                    key: "aimedb",
                    value: aimedb.clone(),
                });
            }
            patches.push(SectionPatch {
                section: "dns",
                entries,
            });
        }

        if let Some(keychip) = &self.keychip {
            patches.push(SectionPatch {
                section: "keychip",
                entries: vec![IniEntry {
                    key: "id",
                    value: keychip.id.clone(),
                }],
            });
        }

        if let Some(vfs) = &self.vfs {
            patches.push(SectionPatch {
                section: "vfs",
                entries: vec![
                    IniEntry { key: "option", value: vfs.option.clone() },
                    IniEntry { key: "amfs", value: vfs.amfs.clone() },
                    IniEntry { key: "appdata", value: vfs.appdata.clone() },
                ],
            });
        }

        if let Some(gpio) = &self.gpio {
            patches.push(SectionPatch {
                section: "gpio",
                entries: vec![
                    IniEntry { key: "dipsw1", value: gpio.dipsw1.to_string() },
                    IniEntry { key: "dipsw2", value: gpio.dipsw2.to_string() },
                    IniEntry { key: "dipsw3", value: gpio.dipsw3.to_string() },
                ],
            });
        }

        if let Some(gfx) = &self.gfx {
            patches.push(SectionPatch {
                section: "gfx",
                entries: vec![
                    IniEntry { key: "windowed", value: gfx.windowed.to_string() },
                    IniEntry { key: "framed", value: gfx.framed.to_string() },
                    IniEntry { key: "monitor", value: gfx.monitor.to_string() },
                ],
            });
        }

        if let Some(aime) = &self.aime {
            patches.push(SectionPatch {
                section: "aime",
                entries: vec![IniEntry {
                    key: "enable",
                    value: if aime.enable { "1".to_string() } else { "0".to_string() },
                }],
            });
        }

        if let Some(io3) = &self.io3 {
            patches.push(SectionPatch {
                section: "io3",
                entries: vec![
                    IniEntry { key: "test", value: io3.test.clone() },
                    IniEntry { key: "service", value: io3.service.clone() },
                    IniEntry { key: "coin", value: io3.coin.clone() },
                ],
            });
        }

        patches
    }
}
