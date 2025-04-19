use std::{collections::HashMap, str::FromStr};

use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Copy)]
pub enum SwapCase {
    PumpFunBuy,
    PumpFunSell,
    PumpAmmBuy,
    PumpAmmSell,
    RaydiumSwap,
    RayLaunchBuy,
    RayLaunchSell
}
#[derive(Debug, Clone, Copy)]
pub struct SwapResult {
    pub case: SwapCase,
    pub amount: u64
}
pub struct SwapData {
    pub pid: Pubkey,
    pub case: SwapCase,
    pub discriminator: Vec<u8>,
    pub amount_pos: usize
}
pub struct Monitor {
    pub swap_data_map: HashMap<Pubkey, Vec<SwapData>>
}
impl Monitor {
    pub const PUMP_PID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
    pub const PUMP_BUY_DISCRIMINATOR: [u8; 8] = [0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea];
    pub const PUMP_SELL_DISCRIMINATOR: [u8; 8] = [0x33, 0xe6, 0x85, 0xa4, 0x01, 0x7f, 0x83, 0xad];

    pub const PUMP_AMM_PID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
    pub const PUMP_AMM_BUY_DISCRIMINATOR: [u8; 8] = [0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea];
    pub const PUMP_AMM_SELL_DISCRIMINATOR: [u8; 8] = [0x33, 0xe6, 0x85, 0xa4, 0x01, 0x7f, 0x83, 0xad];

    pub const RAY_PID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
    pub const RAY_DISCRIMINATOR: [u8; 1] = [0x09];

    pub const RAY_LAUNCH_PID: &str = "LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj";
    pub const RAY_LAUNCH_BUY_DISCRIMINATOR: [u8; 8] = [0xfa, 0xea, 0x0d, 0x7b, 0xd5, 0x9c, 0x13, 0xec];
    pub const RAY_LAUNCH_SELL_DISCRIMINATOR: [u8; 8] = [0x95, 0x27, 0xde, 0x9b, 0xd3, 0x7c, 0x98, 0x1a];

    pub fn create() -> Self {
        let mut swap_data_map = HashMap::new();
        let pump_pid = Pubkey::from_str(Self::PUMP_PID).unwrap();
        swap_data_map.insert(pump_pid, vec![
                SwapData {
                    pid: pump_pid,
                    case: SwapCase::PumpFunBuy,
                    discriminator: Self::PUMP_BUY_DISCRIMINATOR.to_vec(),
                    amount_pos: 8
                },
                SwapData {
                    pid: pump_pid,
                    case: SwapCase::PumpFunSell,
                    discriminator: Self::PUMP_SELL_DISCRIMINATOR.to_vec(),
                    amount_pos: 8
                },
        ]);
        let pump_amm_pid = Pubkey::from_str(Self::PUMP_AMM_PID).unwrap();
        swap_data_map.insert(pump_amm_pid, vec![
            SwapData {
                pid: pump_amm_pid,
                case: SwapCase::PumpAmmBuy,
                discriminator: Self::PUMP_AMM_BUY_DISCRIMINATOR.to_vec(),
                amount_pos: 8
            },
            SwapData {
                pid: pump_amm_pid,
                case: SwapCase::PumpAmmSell,
                discriminator: Self::PUMP_AMM_SELL_DISCRIMINATOR.to_vec(),
                amount_pos: 8
            },
        ]);
        let ray_launch_pid = Pubkey::from_str(Self::RAY_LAUNCH_PID).unwrap();
        swap_data_map.insert(ray_launch_pid, vec![
            SwapData {
                pid: ray_launch_pid,
                case: SwapCase::RayLaunchBuy,
                discriminator: Self::RAY_LAUNCH_BUY_DISCRIMINATOR.to_vec(),
                amount_pos: 8
            },
            SwapData {
                pid: ray_launch_pid,
                case: SwapCase::RayLaunchSell,
                discriminator: Self::RAY_LAUNCH_SELL_DISCRIMINATOR.to_vec(),
                amount_pos: 8
            },
        ]);
        let ray_pool_v4_pid = Pubkey::from_str(Self::RAY_PID).unwrap();
        swap_data_map.insert(ray_pool_v4_pid, vec![
            SwapData {
                pid: ray_pool_v4_pid,
                case: SwapCase::RaydiumSwap,
                discriminator: Self::RAY_DISCRIMINATOR.to_vec(),
                amount_pos: 1
            }
        ]);

        Self {
            swap_data_map
        }

    }
    pub fn detect_swap(&self, pid: &Pubkey, ix_data: &[u8]) -> Option<SwapResult> {
        if self.swap_data_map.contains_key(pid) {
            let swap_data_vec = self.swap_data_map.get(pid).unwrap();
            for swap_data in swap_data_vec {
                if ix_data.starts_with(&swap_data.discriminator) {
                    let amount = u64::from_le_bytes(ix_data[swap_data.amount_pos..(swap_data.amount_pos + 8)].try_into().expect("typecase failed!"));
                    return Some(SwapResult {
                        case: swap_data.case.clone(),
                        amount
                    });
                }
            }
        }
        None
    }
}