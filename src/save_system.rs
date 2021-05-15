use json_db_rs::{Database, JsonDatabase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Save {
    pub highest_score: u32,
}

pub struct SaveSystem<DB>
where
    DB: Database,
{
    database: DB,
}

impl Default for SaveSystem<JsonDatabase> {
    fn default() -> Self {
        Self {
            database: JsonDatabase::default()
        }
    }
}

impl<DB> SaveSystem<DB>
where
    DB: Database,
{
    pub fn load_save(&self) -> Option<Save> {
        let save = self.database.get_all::<Save>();
        if save.len() == 1 {
            save.first().and_then(|x| Some(x.clone()))
        } else {
            None
        }
    }

    pub fn save(&self, save: Save) {
        self.database.save(save);
    }
}
