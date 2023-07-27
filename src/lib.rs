use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, serde::{Serialize, Deserialize}, BorshStorageKey, CryptoHash};
use near_sdk::store::{Vector, UnorderedMap};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Slides {
    deck_map: UnorderedMap<String, UnorderedMap<String, Vector<String>>>,
}

impl Default for Slides {
    fn default() -> Self {
        Self {
            deck_map: UnorderedMap::new(b"d".to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableVector(Vec<String>);

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Account { account_hash: CryptoHash },
    DeckName { deck_name_hash: CryptoHash },
}

#[near_bindgen]
impl Slides {
    #[init]
    #[private]
    pub fn init() -> Self {
        let deck_map: UnorderedMap<String, UnorderedMap<String, Vector<String>>> = UnorderedMap::new(b"d".to_vec());
        Self { deck_map }
    }

    pub fn create_new_deck(&mut self, key: String, deck_name: String) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");

        if !self.deck_map.contains_key(&key) {
            let nested_map: UnorderedMap<String, Vector<String>> =
                UnorderedMap::new(StorageKeys::Account {
                    account_hash: env::sha256_array(caller.as_bytes()),
                });

            self.deck_map.insert(key.clone(), nested_map);
        }

        let nested_map: &mut UnorderedMap<String, Vector<String>> =
            self.deck_map.get_mut(&key).unwrap();

        let deck_name_clone = deck_name.clone();
        if !nested_map.contains_key(&deck_name) {
            nested_map.insert(
                deck_name,
                Vector::new(StorageKeys::DeckName {
                    deck_name_hash: env::sha256_array(deck_name_clone.as_bytes()),
                }),
            );
        }
    }

    pub fn insert_slide(&mut self, key: String, deck_name: String, slide_cid: String) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");

        let nested_map: &mut UnorderedMap<String, Vector<String>> = 
            self.deck_map.get_mut(&key).unwrap();
        let nested_vec: &mut Vector<String> = nested_map.get_mut(&deck_name).unwrap();

        nested_vec.push(slide_cid);
    }

    pub fn insert_slides(&mut self, key: String, deck_name: String, slide_cids: std::vec::Vec<String>) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");

        let nested_map: &mut UnorderedMap<String, Vector<String>> = 
            self.deck_map.get_mut(&key).unwrap();
        let nested_vec: &mut Vector<String> = nested_map.get_mut(&deck_name).unwrap();

        nested_vec.extend(slide_cids);
    }


    pub fn get_slides(&self, key: String, deck_name: String) -> String {
        let nested_map: &UnorderedMap<String, Vector<String>> = self.deck_map.get(&key).unwrap();
        let nested_vec: &Vector<String> = nested_map.get(&deck_name).unwrap();

        if nested_vec.len() != 0  {
            let mut vec: Vec<String> = Vec::with_capacity(nested_vec.len() as usize);
            for element in nested_vec.iter() {
                vec.push(element.clone());
            }
            let serializable_vector = SerializableVector(vec);
            let serialized_vector = serde_json::to_string(&serializable_vector)
                .expect("Serialization error");

            serialized_vector
        } else {
            "None".to_string()
        }
    }
    

    pub fn get_deck_names(&self, key: String) -> String {
        let nested_map: &UnorderedMap<String, Vector<String>> = self.deck_map.get(&key).unwrap();

        let mut vec: Vec<String> = Vec::with_capacity(nested_map.len() as usize);
        for deck_name in nested_map.keys() {
            vec.push(deck_name.clone());
        }
        let serializable_vector = SerializableVector(vec);
        let serialized_vector = serde_json::to_string(&serializable_vector)
            .expect("Serialization error");
        
        serialized_vector
    }

    pub fn get_length_of_nested_vec(&self, key: String, deck_name: String) -> Option<u32> {
        self.deck_map.get(&key).and_then(|nested_map| nested_map.get(&deck_name).map(|vector| vector.len()))
    }

    pub fn get_length_of_nested_map(&self, key: String) -> Option<u32> {
        self.deck_map.get(&key).map(|nested_map| nested_map.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_insert_get() {
        let mut contract: Slides = Slides::default();

        contract.create_new_deck("bob.near".to_string(), "deck 1".to_string());
        contract.insert_slide("bob.near".to_string(), "deck 1".to_string(), "slide 1".to_string());
        contract.insert_slide("bob.near".to_string(), "deck 1".to_string(), "slide 2".to_string());
        contract.insert_slide("bob.near".to_string(), "deck 1".to_string(), "slide 3".to_string());
        contract.insert_slide("bob.near".to_string(), "deck 1".to_string(), "slide 4".to_string());

        assert_eq!(contract.get_slides("bob.near".to_string(), "deck 1".to_string()), "[\"slide 1\",\"slide 2\",\"slide 3\",\"slide 4\"]");
    }

    #[test]
    fn test_insert_slides() {
        let mut contract: Slides = Slides::default();

        contract.create_new_deck("bob.near".to_string(), "deck 1".to_string());
        contract.insert_slide("bob.near".to_string(), "deck 1".to_string(), "slide A".to_string());
        let vec: Vec<String> = vec!["slide B".to_string(), "slide C".to_string(), "slide D".to_string()];
        contract.insert_slides("bob.near".to_string(), "deck 1".to_string(), vec);
        
        assert_eq!(contract.get_slides("bob.near".to_string(), "deck 1".to_string()), "[\"slide A\",\"slide B\",\"slide C\",\"slide D\"]");
    }

    #[test]
    #[should_panic]
    fn panic_insert_slide() {
        let mut contract: Slides = Slides::default();

        contract.create_new_deck("bob.near".to_string(), "deck 1".to_string());
        contract.insert_slide("alice.near".to_string(), "deck 1".to_string(), "slide 1".to_string());
    }

    #[test]
    fn get_deck_names() {
        let mut contract: Slides = Slides::default();

        contract.create_new_deck("bob.near".to_string(), "deck 1".to_string());
        contract.insert_slide("bob.near".to_string(), "deck 1".to_string(), "slide 1A".to_string());
        let vec: Vec<String> = vec!["slide 1B".to_string(), "slide 1C".to_string(), "slide 1D".to_string()];
        contract.insert_slides("bob.near".to_string(), "deck 1".to_string(), vec);

        contract.create_new_deck("bob.near".to_string(), "deck 2".to_string());
        contract.insert_slide("bob.near".to_string(), "deck 2".to_string(), "slide 2A".to_string());

        contract.create_new_deck("bob.near".to_string(), "deck 3".to_string());
        contract.insert_slide("bob.near".to_string(), "deck 3".to_string(), "slide 3A".to_string());

        assert_eq!(contract.get_deck_names("bob.near".to_string()), "[\"deck 1\",\"deck 2\",\"deck 3\"]");        
    }
}