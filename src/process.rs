use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, Read, Write},
    path::PathBuf,
};

use crate::{
    ast::{Character, GroupData, Root},
    config::Config,
};

#[derive(Debug, PartialEq, Clone)]
struct SurfacePart {
    pub number: usize,
    pub digits: usize,
}

type SurfaceNumber = Vec<SurfacePart>;
type SurfacePose = Vec<SurfaceNumber>;
// type SurfaceGroup = Vec<SurfacePose>;

#[derive(Debug)]
pub(crate) enum ProcessError {
    Io(std::io::Error),
    Serde(serde_yml::Error),
}

impl From<std::io::Error> for ProcessError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_yml::Error> for ProcessError {
    fn from(value: serde_yml::Error) -> Self {
        Self::Serde(value)
    }
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::Serde(e) => write!(f, "{}", e),
        }
    }
}

pub(crate) fn process(config: &Config) -> Result<(), ProcessError> {
    if config.output().exists() && !config.force() {
        confirm_overwriting()?;
    }

    let root = read_yaml(config.input())?;

    let contents = build_surfacetable(&root, config.whitelist(), config.separator());

    write_surfacetable(config.output(), &contents)?;

    println!("saved to {}.", config.output().display());

    Ok(())
}

fn confirm_overwriting() -> Result<(), ProcessError> {
    let stdin = std::io::stdin();
    let mut buf_reader = std::io::BufReader::new(stdin);

    let stdout = std::io::stdout();
    let stdout_lock = stdout.lock();
    let mut buf_writer = std::io::BufWriter::new(stdout_lock);

    let mut s = String::new();
    buf_writer.write_all(b"The output file already exists.\n")?;

    loop {
        buf_writer.write_all(b"Do you want to overwrite the file? [Y/n]: ")?;
        buf_writer.flush()?;

        s.clear();
        buf_reader.read_line(&mut s)?;

        match s.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" => {
                buf_writer.write_all(b"The file will be overwritten.\n")?;
                buf_writer.flush()?;
                break;
            }
            "n" | "no" => {
                buf_writer.write_all(b"Closing this program...\n")?;
                buf_writer.flush()?;
                std::process::exit(0);
            }
            _ => {
                buf_writer.write_all(
                    b"Please input 'Y' or 'n' (To exit from this program, input 'n').\n",
                )?;
            }
        }
    }

    Ok(())
}

fn read_yaml(path: &PathBuf) -> Result<Root, ProcessError> {
    let mut fs = File::open(path)?;
    let mut buffer = String::new();
    fs.read_to_string(&mut buffer)?;

    serde_yml::from_str::<Root>(&buffer).map_err(|e| e.into())
}

fn write_surfacetable(path: &PathBuf, contents: &str) -> Result<(), ProcessError> {
    let mut fs = File::create(path)?;
    fs.write_all(contents.as_bytes())?;
    Ok(())
}

fn build_surfacetable(root: &Root, whitelist: Option<&Vec<usize>>, separator: &str) -> String {
    let offset_origin = generate_surface_offset(root.characters());
    let mut contents = r#"charset,UTF-8
version,1
"#
    .to_string();

    for (index, character) in root.characters().iter().enumerate() {
        let tables =
            build_surfacetable_by_character(character, index, offset_origin, whitelist, separator);
        contents.push_str(&tables);
    }

    contents
}

fn build_surfacetable_by_character(
    character: &Character,
    character_index: usize,
    offset: usize,
    whitelist: Option<&Vec<usize>>,
    separator: &str,
) -> String {
    let surfaces = generate_surfaces(character.parts());

    let mut contents = String::new();

    let mut category = String::new();
    for surface_number in surfaces.iter() {
        category.clear();
        let surface_number_base = combine_number(surface_number);
        let surface_number_result = (character_index * offset) + surface_number_base;
        if let Some(list) = whitelist {
            if !list.iter().any(|v| v == &surface_number_result) {
                continue;
            }
        }

        for (index_parts, parts) in surface_number.iter().enumerate() {
            if let Some(v) = character
                .parts()
                .get(index_parts)
                .and_then(|group_data| group_data.details().get(parts.number - 1))
            {
                category.push_str(v.name());
                category.push_str(separator);
            }
        }

        contents.push_str(&format!(
            "{},{}\n",
            surface_number_result,
            category.strip_suffix(separator).unwrap_or(&category)
        ));
    }

    if contents.is_empty() {
        String::new()
    } else {
        format!(
            "\ngroup,\\{}\n{{\nscope,{}\n{}}}\n",
            character_index, character_index, contents
        )
    }
}

fn generate_surface_offset(characters: &[Character]) -> usize {
    let mut max_in_all = 0;

    for c in characters {
        let surfaces = generate_surfaces(c.parts());
        let max = match surfaces.last() {
            Some(v) => combine_number(v),
            None => continue,
        };
        if max > max_in_all {
            max_in_all = max;
        }
    }

    let mut offset = 1;
    let digits = count_digits(max_in_all);
    for _index in 0..digits {
        offset *= 10;
    }

    offset
}

fn generate_surfaces(parts: &[GroupData]) -> SurfacePose {
    const MIN: usize = 1;

    let mut parts_counts = Vec::new();
    let mut digits_counts = Vec::new();

    for p in parts {
        let len = p.details().len();
        parts_counts.push(len);
        digits_counts.push(count_digits(len));
    }

    let mut temp: Vec<SurfacePart> = digits_counts
        .iter()
        .map(|v| SurfacePart {
            number: MIN,
            digits: *v,
        })
        .collect();

    let sum = parts_counts.iter().product();

    let mut surface_numbers = Vec::new();
    for _index_numbers in 0..sum {
        surface_numbers.push(temp.clone());

        for index_temp in (0..parts.len()).rev() {
            temp[index_temp].number += 1;
            if temp[index_temp].number > parts_counts[index_temp] {
                // 郢ｰ繧贋ｸ翫￡縺ｦ邯夊｡・
                temp[index_temp].number = MIN;
            } else {
                // 郢ｰ繧贋ｸ翫′繧翫′縺ｪ縺・・縺ｧ谺｡繝ｫ繝ｼ繝励∈
                break;
            }
        }
    }

    surface_numbers
}

fn count_digits(mut target: usize) -> usize {
    let mut digits = 0;
    while target > 0 {
        target /= 10;
        digits += 1;
    }

    digits
}

fn combine_number(surface_number: &SurfaceNumber) -> usize {
    let mut combined = 0;
    let mut digits = 1;

    for part in surface_number.iter().rev() {
        combined += part.number * digits;
        digits *= 10_usize.pow(part.digits as u32);
    }

    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    mod build_surfacetable {
        use super::*;

        use crate::ast::PoseData;

        #[test]
        fn checking_value() {
            let root = Root::new(
                None,
                vec![
                    Character::new(
                        None,
                        vec![
                            GroupData::new(
                                "testgroup-1_01".to_string(),
                                vec![
                                    PoseData::new("testA01".to_string(), "element0...".to_string()),
                                    PoseData::new("testA02".to_string(), "element0...".to_string()),
                                ],
                            ),
                            GroupData::new(
                                "testgroup-01_02".to_string(),
                                vec![
                                    PoseData::new("testB01".to_string(), "element0...".to_string()),
                                    PoseData::new("testB02".to_string(), "element0...".to_string()),
                                ],
                            ),
                        ],
                    ),
                    Character::new(
                        None,
                        vec![
                            GroupData::new(
                                "testgroup-02_01".to_string(),
                                vec![
                                    PoseData::new("testA01".to_string(), "element0...".to_string()),
                                    PoseData::new("testA02".to_string(), "element0...".to_string()),
                                ],
                            ),
                            GroupData::new(
                                "testgroup-02_02".to_string(),
                                vec![
                                    PoseData::new("testB01".to_string(), "element0...".to_string()),
                                    PoseData::new("testB02".to_string(), "element0...".to_string()),
                                ],
                            ),
                            GroupData::new(
                                "testgroup-02_03".to_string(),
                                vec![
                                    PoseData::new("testC01".to_string(), "element0...".to_string()),
                                    PoseData::new("testC02".to_string(), "element0...".to_string()),
                                ],
                            ),
                        ],
                    ),
                ],
            );
            let separator = "-";
            let result = build_surfacetable(&root, None, &separator);
            assert_eq!(
                result,
                r#"charset,UTF-8
version,1

group,\0
{
scope,0
11,testA01-testB01
12,testA01-testB02
21,testA02-testB01
22,testA02-testB02
}

group,\1
{
scope,1
1111,testA01-testB01-testC01
1112,testA01-testB01-testC02
1121,testA01-testB02-testC01
1122,testA01-testB02-testC02
1211,testA02-testB01-testC01
1212,testA02-testB01-testC02
1221,testA02-testB02-testC01
1222,testA02-testB02-testC02
}
"#
                .to_string()
            );

            let whitelist = vec![11];
            let result = build_surfacetable(&root, Some(&whitelist), &separator);
            assert_eq!(
                result,
                r#"charset,UTF-8
version,1

group,\0
{
scope,0
11,testA01-testB01
}
"#
                .to_string()
            );
        }
    }

    mod build_surfacetable_by_character {
        use super::*;

        use crate::ast::PoseData;

        #[test]
        fn checking_value() {
            let characters = vec![
                Character::new(
                    None,
                    vec![
                        GroupData::new(
                            "testgroup-1_01".to_string(),
                            vec![
                                PoseData::new("testA01".to_string(), "element0...".to_string()),
                                PoseData::new("testA02".to_string(), "element0...".to_string()),
                            ],
                        ),
                        GroupData::new(
                            "testgroup-01_02".to_string(),
                            vec![
                                PoseData::new("testB01".to_string(), "element0...".to_string()),
                                PoseData::new("testB02".to_string(), "element0...".to_string()),
                            ],
                        ),
                    ],
                ),
                Character::new(
                    None,
                    vec![
                        GroupData::new(
                            "testgroup-02_01".to_string(),
                            vec![
                                PoseData::new("testA01".to_string(), "element0...".to_string()),
                                PoseData::new("testA02".to_string(), "element0...".to_string()),
                            ],
                        ),
                        GroupData::new(
                            "testgroup-02_02".to_string(),
                            vec![
                                PoseData::new("testB01".to_string(), "element0...".to_string()),
                                PoseData::new("testB02".to_string(), "element0...".to_string()),
                            ],
                        ),
                        GroupData::new(
                            "testgroup-02_03".to_string(),
                            vec![
                                PoseData::new("testC01".to_string(), "element0...".to_string()),
                                PoseData::new("testC02".to_string(), "element0...".to_string()),
                            ],
                        ),
                    ],
                ),
            ];
            let offset = generate_surface_offset(&characters);
            let character_index = 0;
            let separator = "-";

            let result = build_surfacetable_by_character(
                &characters[character_index],
                character_index,
                offset,
                None,
                &separator,
            );

            assert_eq!(
                result,
                r#"
group,\0
{
scope,0
11,testA01-testB01
12,testA01-testB02
21,testA02-testB01
22,testA02-testB02
}
"#
                .to_string()
            );

            let offset = generate_surface_offset(&characters);
            let character_index = 1;
            let whitelist = vec![1111, 1211, 1222];

            let result = build_surfacetable_by_character(
                &characters[character_index],
                character_index,
                offset,
                Some(&whitelist),
                &separator,
            );

            assert_eq!(
                result,
                r#"
group,\1
{
scope,1
1111,testA01-testB01-testC01
1211,testA02-testB01-testC01
1222,testA02-testB02-testC02
}
"#
                .to_string()
            );
        }
    }

    mod generate_surface_offset {
        use super::*;

        use crate::ast::PoseData;

        #[test]
        fn checking_value() {
            let case = vec![
                Character::new(
                    None,
                    vec![
                        GroupData::new(
                            "testgroup-1_01".to_string(),
                            vec![
                                PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                                PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                            ],
                        ),
                        GroupData::new(
                            "testgroup-01_02".to_string(),
                            vec![
                                PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                                PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                            ],
                        ),
                    ],
                ),
                Character::new(
                    None,
                    vec![
                        GroupData::new(
                            "testgroup-02_01".to_string(),
                            vec![
                                PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                                PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                            ],
                        ),
                        GroupData::new(
                            "testgroup-02_02".to_string(),
                            vec![
                                PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                                PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                            ],
                        ),
                        GroupData::new(
                            "testgroup-02_03".to_string(),
                            vec![
                                PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                                PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                            ],
                        ),
                    ],
                ),
            ];
            let result = generate_surface_offset(&case);
            assert_eq!(result, 1000);
        }
    }

    mod generate_surfaces {
        use super::*;

        use crate::ast::PoseData;

        #[test]
        fn checking_value() {
            let case = vec![
                GroupData::new(
                    "testgroup_01".to_string(),
                    vec![
                        PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                        PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                    ],
                ),
                GroupData::new(
                    "testgroup_02".to_string(),
                    vec![
                        PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                        PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                    ],
                ),
            ];
            let result = generate_surfaces(&case);
            assert_eq!(
                result,
                vec![
                    vec![
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        }
                    ],
                    vec![
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 2,
                            digits: 1
                        }
                    ],
                    vec![
                        SurfacePart {
                            number: 2,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        }
                    ],
                    vec![
                        SurfacePart {
                            number: 2,
                            digits: 1
                        },
                        SurfacePart {
                            number: 2,
                            digits: 1
                        }
                    ]
                ]
            );

            let case = vec![
                GroupData::new(
                    "testgroup_01".to_string(),
                    vec![PoseData::new(
                        "testpose_01".to_string(),
                        "element0...".to_string(),
                    )],
                ),
                GroupData::new(
                    "testgroup_02".to_string(),
                    vec![
                        PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                        PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                        PoseData::new("testpose_03".to_string(), "element0...".to_string()),
                    ],
                ),
            ];
            let result = generate_surfaces(&case);
            assert_eq!(
                result,
                vec![
                    vec![
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        }
                    ],
                    vec![
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 2,
                            digits: 1
                        }
                    ],
                    vec![
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 3,
                            digits: 1
                        }
                    ],
                ]
            );

            let case = vec![
                GroupData::new(
                    "testgroup_02".to_string(),
                    vec![
                        PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                        PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                        PoseData::new("testpose_03".to_string(), "element0...".to_string()),
                    ],
                ),
                GroupData::new(
                    "testgroup_01".to_string(),
                    vec![PoseData::new(
                        "testpose_01".to_string(),
                        "element0...".to_string(),
                    )],
                ),
            ];
            let result = generate_surfaces(&case);
            assert_eq!(
                result,
                vec![
                    vec![
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        }
                    ],
                    vec![
                        SurfacePart {
                            number: 2,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                    ],
                    vec![
                        SurfacePart {
                            number: 3,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                    ],
                ]
            );

            let case = vec![
                GroupData::new(
                    "testgroup_01".to_string(),
                    vec![PoseData::new(
                        "testpose_01".to_string(),
                        "element0...".to_string(),
                    )],
                ),
                GroupData::new(
                    "testgroup_02".to_string(),
                    vec![
                        PoseData::new("testpose_01".to_string(), "element0...".to_string()),
                        PoseData::new("testpose_02".to_string(), "element0...".to_string()),
                    ],
                ),
                GroupData::new(
                    "testgroup_03".to_string(),
                    vec![PoseData::new(
                        "testpose_01".to_string(),
                        "element0...".to_string(),
                    )],
                ),
            ];
            let result = generate_surfaces(&case);
            assert_eq!(
                result,
                vec![
                    vec![
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                    ],
                    vec![
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                        SurfacePart {
                            number: 2,
                            digits: 1
                        },
                        SurfacePart {
                            number: 1,
                            digits: 1
                        },
                    ],
                ]
            );
        }
    }

    mod count_digits {
        use super::*;

        #[test]
        fn cheking_value() {
            let case = 222;
            assert_eq!(count_digits(case), 3);

            let case = 1;
            assert_eq!(count_digits(case), 1);

            let case = 0;
            assert_eq!(count_digits(case), 0);
        }
    }

    mod combine_number {
        use super::*;

        #[test]
        fn checking_value() {
            let case: SurfaceNumber = vec![
                SurfacePart {
                    number: 3,
                    digits: 1,
                },
                SurfacePart {
                    number: 5,
                    digits: 2,
                },
            ];
            let result = combine_number(&case);
            assert_eq!(result, 305);
        }
    }
}
