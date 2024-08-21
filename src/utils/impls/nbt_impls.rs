use byteorder::WriteBytesExt;
use futures::AsyncWriteExt;
use crate::utils::error::Error;
use crate::utils::impls::type_impls::Encode;
use nbt_lib::{NBTSerialize, NBTTag};
use tokio::io::{AsyncSeek, AsyncWrite};

impl Encode for NBTTag {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let mut pseudo_sync_cursor = Vec::new();
        // bytes.write_u8(self.tag_type()).await?;
        // Empty ROOT name
        // bytes.write_all(&[0; 4]).await?;
        // self.serialize(bytes).await?;

        pseudo_sync_cursor.write_u8(self.tag_type())?;
        pseudo_sync_cursor.write_all(&[0; 4]).await?;
        self.serialize(&mut pseudo_sync_cursor)?;

        {
            use tokio::io::AsyncWriteExt;
            bytes.write_all(&pseudo_sync_cursor).await?;
        }
        Ok(())
    }
}