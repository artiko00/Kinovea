//! Image options stored by name in KVA files: ports of `Kinovea.Services/Types/ImageRotation.cs`
//! and `Demosaicing.cs`.

crate::text_enum!(
    /// `ImageRotation`: rotation flag stored in the file.
    ImageRotation [Rotate0, Rotate90, Rotate180, Rotate270]
);

crate::text_enum!(
    /// `Demosaicing`: Bayer pattern used to decode raw sensor data.
    Demosaicing [None, RGGB, BGGR, GRBG, GBRG]
);
