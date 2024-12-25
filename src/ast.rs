use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Root {
    raw: Option<String>,
    characters: Vec<Character>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Character {
    base: Option<String>,
    parts: Vec<GroupData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct GroupData {
    group: String,
    details: Vec<PoseData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PoseData {
    name: String,
    text: String,
}

impl Root {
    #[cfg(test)]
    pub fn new(raw: Option<String>, characters: Vec<Character>) -> Root {
        Root { raw, characters }
    }

    #[cfg(test)]
    pub fn raw(&self) -> Option<&String> {
        self.raw.as_ref()
    }

    pub fn characters(&self) -> &Vec<Character> {
        &self.characters
    }
}

impl Character {
    #[cfg(test)]
    pub fn new(base: Option<String>, parts: Vec<GroupData>) -> Character {
        Character { base, parts }
    }

    #[cfg(test)]
    pub fn base(&self) -> Option<&String> {
        self.base.as_ref()
    }

    pub fn parts(&self) -> &Vec<GroupData> {
        &self.parts
    }
}

impl GroupData {
    #[cfg(test)]
    pub fn new(group: String, details: Vec<PoseData>) -> GroupData {
        GroupData { group, details }
    }
    #[allow(dead_code)]
    pub fn group(&self) -> &String {
        &self.group
    }

    pub fn details(&self) -> &Vec<PoseData> {
        &self.details
    }
}

impl PoseData {
    #[cfg(test)]
    pub fn new(name: String, text: String) -> PoseData {
        PoseData { name, text }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    #[cfg(test)]
    pub fn text(&self) -> &String {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod deserialise {
        use super::*;

        #[test]
        fn success_when_valid_str_01() {
            let case = r#"
raw: |
  descript
  {
    version,1
  }

characters:
  # \0側キャラクター
  - parts:
    - group: 顔色
      details:
        - name: 通常顔
          text: |

        - name: 照れ顔
          text: |
            animation500600.interval,runonce
            animation500600.pattern0,overlay,101,0,0,0

    - group: 目
      details:
        - name: こっち目
          text: |
            animation500300.interval,runonce+rarely
            animation500300.option,shared-index
            animation500300.pattern0,overlay,201,0,0,0
            animation500300.pattern1,overlay,209,4000,0,0
            animation500300.pattern2,overlay,204,100,0,0
            animation500300.pattern3,overlay,201,100,0,0

        - name: あっち目
          text: |
            animation500300.interval,runonce+rarely
            animation500300.option,shared-index
            animation500300.pattern0,overlay,203,0,0,0
            animation500300.pattern1,overlay,209,4000,0,0
            animation500300.pattern2,overlay,206,100,0,0
            animation500300.pattern3,overlay,203,100,0,0

    - group: 腕
      details:
        - name: 前手
          text: |
            animation505000.interval,runonce
            animation505000.pattern0,overlay,503,0,0,0

            collisionex13,hand,polygon,288,423,330,410,336,414,341,422,328,446,317,436,288,442
            collisionex14,hand,polygon,282,501,262,536,261,550,285,556,299,549,294,521,295,512

        - name: 胸に手
          text: |
            animation504000.interval,runonce
            animation504000.pattern0,overlay,501,0,0,0

            animation590000.interval,runonce
            animation590000.pattern0,overlay,601,0,0,0

            collisionex13,hand,polygon,288,423,330,410,336,414,341,422,328,446,317,436,288,442
            collisionex14,hand,polygon,282,501,262,536,261,550,285,556,299,549,294,521,295,512

    base: |
     collisionex10,shoulder,polygon,205,319,206,309,214,301,229,299,251,293,248,312
     collisionex11,shoulder,polygon,292,293,309,299,332,302,339,315,301,315
     collisionex7,mouse,ellipse,260,259,283,268
     collisionex8,head,polygon,292,134,319,151,340,176,339,192,292,190,240,192,201,187,213,158,230,140,259,127
     collisionex6,face,polygon,270,285,239,267,227,246,220,214,222,189,227,173,315,170,320,189,321,210,315,245,305,268

     //素体
     element0,overlay,surface1000.png,0,0


  # \1側キャラクター
  - parts:
    - group: 素体
      details:
        - name: 通常
          text: |

        - name: 腕上げ
          text: |
            animation500600.interval,runonce
            animation500600.pattern0,overlay,101,0,0,0

    - group: 目
      details:
        - name: こっち目
          text: |
            element0,overlay,surface3000.png,0,0

        - name: あっち目
          text: |
            element0,overlay,surface3001.png,0,0
"#;
            let result: Root = serde_yml::from_str(&case).unwrap();
            assert!(result.characters[1].base.is_none());
            assert_eq!(result.characters[1].parts[0].group, "素体");
        }

        #[test]
        fn success_when_valid_str_02() {
            let case = r#"
characters:
  - base: |
      hogehoge
    parts:
      - group:
        details:
          - name: bbb
            text: |
"#;
            let result: Root = serde_yml::from_str(&case).unwrap();
            assert!(result.raw.is_none());
        }

        #[test]
        fn failed_when_invalid_str() {
            let case = r#"
characters:
  - base: |
      hogehoge
    parts:
      - group:
"#;
            assert!(serde_yml::from_str::<Root>(&case).is_err());
        }
    }
}
