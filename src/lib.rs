use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, serde::{Serialize, Deserialize}};
use near_sdk::store::{Vector, UnorderedMap};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Slides {
    unordered_map_vec: UnorderedMap<String, Vector<String>>,
    unordered_map_map: UnorderedMap<String, UnorderedMap<String, Vector<String>>>,
}

impl Default for Slides {
    fn default() -> Self {
        Self {
            unordered_map_vec: UnorderedMap::new(b"c".to_vec()),
            unordered_map_map: UnorderedMap::new(b"d".to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableVector(Vec<String>);

#[near_bindgen]
impl Slides {
    #[init]
    #[private]
    pub fn init() -> Self {
        let mut unordered_map_vec: UnorderedMap<String, Vector<String>> = UnorderedMap::new(b"c".to_vec());
        let mut unordered_map_map: UnorderedMap<String, UnorderedMap<String, Vector<String>>> = UnorderedMap::new(b"d".to_vec());

        Self { unordered_map_vec, unordered_map_map }
    }

    // UnorderedMap<String, Vector<String>>
    pub fn unordered_map_vec_init(&mut self, key: String) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");

        let nested = self.unordered_map_vec.get_mut(&key);
        if nested.is_none() {
            self.unordered_map_vec.insert(key, Vector::new(b"c".to_vec()));
        } else {
            return;
        }
    }

    pub fn unordered_map_vec_insert(&mut self, key: String, insert_value: String) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");

        let mut nested = self.unordered_map_vec.get_mut(&key).unwrap();
        nested.push(insert_value);
    }

    pub fn unordered_map_vec_extend(&mut self, key: String, insert_values: std::vec::Vec<String>) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");

        let mut nested = self.unordered_map_vec.get_mut(&key).unwrap();
        nested.extend(insert_values);
    }
    
    pub fn unordered_map_vec_get_ser(&self, key: String) -> String {
        let nested = self.unordered_map_vec.get(&key).unwrap(); 
        let mut vec: Vec<String> = Vec::with_capacity(nested.len() as usize);
        for element in nested.iter() {
            vec.push(element.clone());
        }
        let serializable_vector = SerializableVector(vec);
        let serialized_vector = serde_json::to_string(&serializable_vector).expect("Serialization error");
        serialized_vector
    }

    // UnorderedMap<String, UnorderedMap<String, Vector<String>>>
    pub fn unordered_map_map_init(&mut self, key: String, deck_name: String) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");

        let mut nested_map: UnorderedMap<String, Vector<String>> = UnorderedMap::new(b"c".to_vec());
        nested_map.insert(deck_name, Vector::new(b"d".to_vec()));
        self.unordered_map_map.insert(key, nested_map);
    }

    pub fn unordered_map_map_insert(&mut self, key: String, deck_name: String, slide_cid: String) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");

        let mut nested_map: &mut UnorderedMap<String, Vector<String>> = self.unordered_map_map.get_mut(&key).unwrap();
        let mut nested_vec: &mut Vector<String> = nested_map.get_mut(&deck_name).unwrap();
        nested_vec.push(slide_cid);
    }

    pub fn unordered_map_map_extend(&mut self, key: String, deck_name: String, slide_cids: std::vec::Vec<String>) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");


        let mut nested_map: &mut UnorderedMap<String, Vector<String>> = self.unordered_map_map.get_mut(&key).unwrap();
        let mut nested_vec: &mut Vector<String> = nested_map.get_mut(&deck_name).unwrap();
        nested_vec.extend(slide_cids);
    }

    pub fn unordered_map_map_get_ser(&self, key: String, deck_name: String) -> String {
        let nested_map: &UnorderedMap<String, Vector<String>> = self.unordered_map_map.get(&key).unwrap();
        let nested_vec: &Vector<String> = nested_map.get(&deck_name).unwrap();

        let mut vec: Vec<String> = Vec::with_capacity(nested_vec.len() as usize);
        for element in nested_vec.iter() {
            vec.push(element.clone());
        }
        let serializable_vector = SerializableVector(vec);
        let serialized_vector = serde_json::to_string(&serializable_vector).expect("Serialization error");
        serialized_vector
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_vec() {
        let mut contract: Slides = Slides::default();
        contract.unordered_map_vec_init("bob.near".to_string());
        contract.unordered_map_vec_insert("bob.near".to_string(), "insert 1".to_string());
        contract.unordered_map_vec_insert("bob.near".to_string(), "insert 2".to_string());
        contract.unordered_map_vec_insert("bob.near".to_string(), "insert 3".to_string());
        assert_eq!(contract.unordered_map_vec_get_ser("bob.near".to_string()), "[\"insert 1\",\"insert 2\",\"insert 3\"]");
    }

    #[test]
    #[should_panic]
    fn panic_map_vec_init() {
        let mut contract: Slides = Slides::default();
        contract.unordered_map_vec_init("alice.near".to_string());
    }

    #[test]
    #[should_panic]
    fn panic_map_vec_insert() {
        let mut contract: Slides = Slides::default();
        contract.unordered_map_vec_init("bob.near".to_string());
        contract.unordered_map_vec_insert("alice.near".to_string(), "insert 1".to_string());
    }

    #[test]
    fn test_unordered_map_vec_extend() {
        let mut contract: Slides = Slides::default();
        contract.unordered_map_vec_init("bob.near".to_string());
        contract.unordered_map_vec_insert("bob.near".to_string(), "0xFirstValue".to_string());

        let vec: Vec<String> = vec!["0xSecondValue".to_string(), "0xThirdValue".to_string(), "0xFourthValue".to_string()];

        contract.unordered_map_vec_extend("bob.near".to_string(),vec );
        assert_eq!(contract.unordered_map_vec_get_ser("bob.near".to_string()), "[\"0xFirstValue\",\"0xSecondValue\",\"0xThirdValue\",\"0xFourthValue\"]");
    }

    #[test]
    fn test_unordered_map_map() {
        let mut contract: Slides = Slides::default();
        contract.unordered_map_map_init("bob.near".to_string(), "deck 1".to_string());
        contract.unordered_map_map_insert("bob.near".to_string(), "deck 1".to_string(), "slide 1".to_string());
        contract.unordered_map_map_insert("bob.near".to_string(), "deck 1".to_string(), "slide 2".to_string());
        contract.unordered_map_map_insert("bob.near".to_string(), "deck 1".to_string(), "slide 3".to_string());
        contract.unordered_map_map_insert("bob.near".to_string(), "deck 1".to_string(), "slide 4".to_string());

        assert_eq!(contract.unordered_map_map_get_ser("bob.near".to_string(), "deck 1".to_string()), "[\"slide 1\",\"slide 2\",\"slide 3\",\"slide 4\"]");
    }

    #[test]
    fn test_unordered_map_map_extend() {
        let mut contract: Slides = Slides::default();
        contract.unordered_map_map_init("bob.near".to_string(), "deck 1".to_string());
        contract.unordered_map_map_insert("bob.near".to_string(), "deck 1".to_string(), "slide A".to_string());

        let vec: Vec<String> = vec!["slide B".to_string(), "slide C".to_string(), "slide D".to_string()];

        contract.unordered_map_map_extend("bob.near".to_string(), "deck 1".to_string(), vec);
        assert_eq!(contract.unordered_map_map_get_ser("bob.near".to_string(), "deck 1".to_string()), "[\"slide A\",\"slide B\",\"slide C\",\"slide D\"]");
    }

}