// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Instant Seal

use sc_consensus::BlockImport;
use sp_runtime::traits::Block as BlockT;

pub struct InstantFinalizeBlockImport<I>(I);

impl<I> InstantFinalizeBlockImport<I> {
    /// Create a new instance.
    pub fn new(inner: I) -> Self {
        Self(inner)
    }
}

#[async_trait::async_trait]
impl<Block, I> BlockImport<Block> for InstantFinalizeBlockImport<I>
where
    Block: BlockT,
    I: BlockImport<Block> + Send,
{
    type Error = I::Error;
    type Transaction = I::Transaction;

    async fn check_block(
        &mut self,
        block: sc_consensus::BlockCheckParams<Block>,
    ) -> Result<sc_consensus::ImportResult, Self::Error> {
        self.0.check_block(block).await
    }

    async fn import_block(
        &mut self,
        mut block_import_params: sc_consensus::BlockImportParams<Block, Self::Transaction>,
    ) -> Result<sc_consensus::ImportResult, Self::Error> {
        block_import_params.finalized = true;
        self.0.import_block(block_import_params).await
    }
}
