use crate::pubkeys::PubKeys;
use crate::relay_manager::RelayManager;
use crate::stats::Stats;
use nostr_sdk::prelude::{Event, FromBech32, Keys, Kind, Result, SecretKey, Tag, Timestamp};

mod pubkeys;
mod relay_manager;
mod relays;
mod stats;

const BOOTSTRAP_RELAY1: &str = "wss://nos.lol";
const BOOTSTRAP_RELAY2: &str = "wss://relay.damus.io";
const BOOTSTRAP_RELAY3: &str = "wss://relay.plebstr.com";
const APP_SECRET_KEY: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";

pub struct Processor {
    pubkeys: PubKeys,
    stats: Stats,
}

impl Processor {
    fn default() -> Self {
        Self {
            pubkeys: PubKeys::default(),
            stats: Stats::default(),
        }
    }

    #[allow(dead_code)]
    fn age(t: Timestamp) -> i64 {
        Timestamp::now().as_i64() - t.as_i64()
    }

    pub fn handle_event(&mut self, event: &Event) {
        // println!("{:?}", event);
        // println!("age {:?}  created_at {:?}", Self::age(event.created_at), event.created_at);
        match event.kind {
            Kind::ContactList => {
                self.stats.add_contacts();
                // count p tags
                let mut cnt = 0;
                for t in &event.tags {
                    if let Tag::PubKey(pk, _s) = t {
                        self.pubkeys.add(pk);
                        cnt = cnt + 1;
                    }
                }
                println!("Contacts {} \t ", cnt); // event.pubkey.to_bech32().unwrap(),
                                                  // self.print_summary();
            }
            Kind::RecommendRelay => {
                self.stats.add_relays();
            }
            _ => {
                // println!("Unsupported event {:?}", event.kind)
            }
        }
    }

    // fn print_summary(&self) {
    //     print!("pks {} \t ", self.pubkeys.count());
    //     self.stats.print_summary();
    // }

    pub fn dump(&self) {
        println!();
        println!(
            "Number of ContactList events:      \t {}",
            self.stats.count_contacts
        );
        println!(
            "Number of RecommendedRelay events: \t {}",
            self.stats.count_relays
        );
        println!();
        self.pubkeys.dump();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app_secret_key = SecretKey::from_bech32(APP_SECRET_KEY)?;
    let app_keys = Keys::new(app_secret_key);
    let processor = Processor::default();
    let mut relay_manager = RelayManager::new(app_keys, processor);
    relay_manager
        .run(vec![BOOTSTRAP_RELAY1, BOOTSTRAP_RELAY2, BOOTSTRAP_RELAY3])
        .await?;
    relay_manager.processor.dump();

    Ok(())
}
