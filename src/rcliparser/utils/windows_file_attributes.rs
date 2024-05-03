
#[derive(PartialEq, Eq)]
pub enum WindowsAttributes{
    READONLY,
    HIDDEN,
    FILE_SYSTEM,
    DIRECTORY,
    ARCHIVE,
    DEVICE,
    NORMAL,
    TEMPORARY,
    SPARSE_FILE,
    REPARSE_POINT,
    COMPRESSED,
    OFFLINE,
    NOT_CONTENT_INDEXED,
    ENCRYPTED,
    INTEGRITY_STREAM,
    VIRTUAL,
    NO_SCRUB_DATA,
    INTERNAL_ATTRIBUTES,
    PINNED
}


pub fn match_attributes(attribute_value: u32) -> Vec<WindowsAttributes>{

    let mut windows_attributes = Vec::<WindowsAttributes>::new();

    if (attribute_value & 0x1) > 0 {
        windows_attributes.push(WindowsAttributes::READONLY);
    }
    if (attribute_value & 0x2) > 0 {
        windows_attributes.push(WindowsAttributes::HIDDEN);
    }
    if (attribute_value & 0x4) > 0 {
        windows_attributes.push(WindowsAttributes::FILE_SYSTEM);
    }
    if (attribute_value & 0x16) > 0 {
        windows_attributes.push(WindowsAttributes::DIRECTORY);
    }
    if (attribute_value & 0x32) > 0 {
        windows_attributes.push(WindowsAttributes::ARCHIVE);
    }
    if (attribute_value & 0x64) > 0 {
        windows_attributes.push(WindowsAttributes::DEVICE);
    }
    if (attribute_value & 0x128) > 0 {
        windows_attributes.push(WindowsAttributes::NORMAL);
    }
    if (attribute_value & 0x256) > 0 {
        windows_attributes.push(WindowsAttributes::TEMPORARY);
    }
    if (attribute_value & 0x512) > 0 {
        windows_attributes.push(WindowsAttributes::SPARSE_FILE);
    }
    if (attribute_value & 0x1024) > 0 {
        windows_attributes.push(WindowsAttributes::REPARSE_POINT);
    }
    if (attribute_value & 0x2048) > 0 {
        windows_attributes.push(WindowsAttributes::COMPRESSED);
    }
    if (attribute_value & 0x4096) > 0 {
        windows_attributes.push(WindowsAttributes::OFFLINE);
    }
    if (attribute_value & 0x8192) > 0 {
        windows_attributes.push(WindowsAttributes::NOT_CONTENT_INDEXED);
    }
    if (attribute_value & 0x16384) > 0 {
        windows_attributes.push(WindowsAttributes::ENCRYPTED);
    }
    if (attribute_value & 0x32768) > 0 {
        windows_attributes.push(WindowsAttributes::INTEGRITY_STREAM);
    }
    if (attribute_value & 0x65536) > 0 {
        windows_attributes.push(WindowsAttributes::VIRTUAL);
    }
    if (attribute_value & 0x131072) > 0 {
        windows_attributes.push(WindowsAttributes::NO_SCRUB_DATA);
    }
    if (attribute_value & 0x262144) > 0 {
        windows_attributes.push(WindowsAttributes::INTERNAL_ATTRIBUTES);
    }
    if (attribute_value & 0x524288) > 0 {
        windows_attributes.push(WindowsAttributes::PINNED);
    }

    return windows_attributes
}