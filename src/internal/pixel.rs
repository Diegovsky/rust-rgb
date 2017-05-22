use std;

/// Casting the struct to slices/bytes of its components
pub trait ComponentBytes<T> {
    /// The components interpreted as an array, e.g. RGB gives 3-element slice. The red component is first.
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];

    /// The components interpreted as raw bytes, in machine's native endian. Bytes of the red component are first.
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        let slice = self.as_slice();
        unsafe {
            std::slice::from_raw_parts(std::mem::transmute(slice.as_ptr()), slice.len() * std::mem::size_of::<T>())
        }
    }
}

/// Cast array of structs into array of raw bytes (so that `[0]` is R, `[1]` is G, `[2]` is B, etc.)
///
/// Escape hatch for interoperability with C and non-type-safe libraries.
pub trait ByteSlice {
    /// Read-only view of bytes. `[0]` is R, `[1]` is G, `[2]` is B, etc.
    fn as_bytes(&self) -> &[u8];
    /// Read-write view of bytes.
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

/// Applying operation to every component
///
/// ```rust,ignore
/// let inverted = pixel.map(|c| 255 - c);
///
/// For simple math there are Add/Sub/Mul implementations:
/// let halved = pixel.map(|c| c / 2);
/// let halved = pixel / 2;
/// ```
pub trait ComponentMap<DestPixel, SrcComponent, DestComponent> {
    /// Convenience function (equivalent of `self.iter().map().collect()`) for applying the same formula to every component.
    ///
    /// Note that it returns the pixel directly, not an Interator.
    fn map<Callback>(&self, f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent;
}