# [repr (packed)]
# [derive (Clone , Copy)]
pub struct FrameCounterNone < > where { } impl < > FrameCounterNone < > where { # [inline (always)]
pub fn new () -> Self { Self { } } } # [repr (packed)]
# [derive (Clone , Copy)]
pub struct FrameCounterPresent < > where { frame_counter : u32 , } pub struct FrameCounter < 'a , > where { data : & 'a mut FrameCounterPresent < > } impl < 'a , > FrameCounter < 'a , > where { # [inline (always)]
pub (crate) fn new (data : & 'a mut FrameCounterPresent < >) -> Self { Self { data } } # [inline (always)]
pub fn read (& self) -> u32 { self . data . frame_counter } # [inline (always)]
pub fn set (& 'a mut self , v : u32) -> & 'a mut FrameCounterPresent < > { self . data . frame_counter = v ; self . data } } impl < > FrameCounterPresent < > where { # [inline (always)]
pub fn new () -> Self { Self { frame_counter : 0 , } } pub fn frame_counter (& mut self) -> FrameCounter < > { FrameCounter :: new (self) } } # [repr (packed)]
# [derive (Clone , Copy)]
pub struct KeyIdNone < > where { } impl < > KeyIdNone < > where { # [inline (always)]
pub fn new () -> Self { Self { } } } # [repr (packed)]
# [derive (Clone , Copy)]
pub struct KeyIdOnly < > where { key_id : u8 , } pub struct KeyId < 'a , > where { data : & 'a mut KeyIdOnly < > } impl < 'a , > KeyId < 'a , > where { # [inline (always)]
pub (crate) fn new (data : & 'a mut KeyIdOnly < >) -> Self { Self { data } } # [inline (always)]
pub fn read (& self) -> u8 { self . data . key_id } # [inline (always)]
pub fn set (& 'a mut self , v : u8) -> & 'a mut KeyIdOnly < > { self . data . key_id = v ; self . data } } impl < > KeyIdOnly < > where { # [inline (always)]
pub fn new () -> Self { Self { key_id : 0 , } } pub fn key_id (& mut self) -> KeyId < > { KeyId :: new (self) } } # [repr (packed)]
# [derive (Clone , Copy)]
pub struct KeyIdShort < > where { key_source : u32 , key_id : u8 , } pub struct KeySource < 'a , > where { data : & 'a mut KeyIdShort < > } impl < 'a , > KeySource < 'a , > where { # [inline (always)]
pub (crate) fn new (data : & 'a mut KeyIdShort < >) -> Self { Self { data } } # [inline (always)]
pub fn read (& self) -> u32 { self . data . key_source } # [inline (always)]
pub fn set (& 'a mut self , v : u32) -> & 'a mut KeyIdShort < > { self . data . key_source = v ; self . data } } pub struct KeyId < 'a , > where { data : & 'a mut KeyIdShort < > } impl < 'a , > KeyId < 'a , > where { # [inline (always)]
pub (crate) fn new (data : & 'a mut KeyIdShort < >) -> Self { Self { data } } # [inline (always)]
pub fn read (& self) -> u8 { self . data . key_id } # [inline (always)]
pub fn set (& 'a mut self , v : u8) -> & 'a mut KeyIdShort < > { self . data . key_id = v ; self . data } } impl < > KeyIdShort < > where { # [inline (always)]
pub fn new () -> Self { Self { key_source : 0 , key_id : 0 , } } pub fn key_source (& mut self) -> KeySource < > { KeySource :: new (self) } pub fn key_id (& mut self) -> KeyId < > { KeyId :: new (self) } } # [repr (packed)]
# [derive (Clone , Copy)]
pub struct KeyIdLong < > where { key_source : u64 , key_id : u8 , } pub struct KeySource < 'a , > where { data : & 'a mut KeyIdLong < > } impl < 'a , > KeySource < 'a , > where { # [inline (always)]
pub (crate) fn new (data : & 'a mut KeyIdLong < >) -> Self { Self { data } } # [inline (always)]
pub fn read (& self) -> u64 { self . data . key_source } # [inline (always)]
pub fn set (& 'a mut self , v : u64) -> & 'a mut KeyIdLong < > { self . data . key_source = v ; self . data } } pub struct KeyId < 'a , > where { data : & 'a mut KeyIdLong < > } impl < 'a , > KeyId < 'a , > where { # [inline (always)]
pub (crate) fn new (data : & 'a mut KeyIdLong < >) -> Self { Self { data } } # [inline (always)]
pub fn read (& self) -> u8 { self . data . key_id } # [inline (always)]
pub fn set (& 'a mut self , v : u8) -> & 'a mut KeyIdLong < > { self . data . key_id = v ; self . data } } impl < > KeyIdLong < > where { # [inline (always)]
pub fn new () -> Self { Self { key_source : 0 , key_id : 0 , } } pub fn key_source (& mut self) -> KeySource < > { KeySource :: new (self) } pub fn key_id (& mut self) -> KeyId < > { KeyId :: new (self) } } pub trait KeyId : Copy { fn default () -> Self ; } enum KeyIdA { KeyIdNone , KeyIdOnly , KeyIdShort , KeyIdLong , } pub trait FrameCounter : Copy { fn default () -> Self ; } enum FrameCounterA { FrameCounterNone , FrameCounterNone , } impl KeyId for KeyIdNone { fn default () -> Self { Self :: new () } } impl KeyId for KeyIdOnly { fn default () -> Self { Self :: new () } } impl KeyId for KeyIdShort { fn default () -> Self { Self :: new () } } impl KeyId for KeyIdLong { fn default () -> Self { Self :: new () } } impl FrameCounter for FrameCounterNone { fn default () -> Self { Self :: new () } } impl FrameCounter for FrameCounterNone { fn default () -> Self { Self :: new () } } # [repr (packed)]
# [derive (Clone , Copy)]
pub struct SecurityControl < FrameCounterT , > where FrameCounterT : FrameCounter , { security_control : u8 , frame_counter : FrameCounterT , } pub struct SecurityControl < 'a , FrameCounterT , > where FrameCounterT : FrameCounter , { data : & 'a mut SecurityControl < FrameCounterT , > } impl < 'a , FrameCounterT , > SecurityControl < 'a , FrameCounterT , > where FrameCounterT : FrameCounter , { # [inline (always)]
pub (crate) fn new (data : & 'a mut SecurityControl < FrameCounterT , >) -> Self { Self { data } } # [inline (always)]
pub fn read (& self) -> crate :: security_control :: R { crate :: security_control :: R :: new (self . data . security_control) } # [inline (always)]
pub fn modify < F > (& 'a mut self , f : F) -> & 'a mut SecurityControl < FrameCounterT , > where for < 'w > F : FnOnce (& 'w mut crate :: security_control :: W) -> & 'w mut crate :: security_control :: W { let bits = self . data . security_control ; self . data . security_control = * * f (& mut crate :: security_control :: W :: new (bits)) ; self . data } } pub struct FrameCounter < 'a , FrameCounterT , > where FrameCounterT : FrameCounter , { data : & 'a mut SecurityControl < FrameCounterT , > } impl < 'a , FrameCounterT , > FrameCounter < 'a , FrameCounterT , > where FrameCounterT : FrameCounter , { # [inline (always)]
pub (crate) fn new (data : & 'a mut SecurityControl < FrameCounterT , >) -> Self { Self { data } } # [inline (always)]
pub fn read (& self) -> FrameCounterT { self . data . frame_counter } # [inline (always)]
pub fn modify < F > (& 'a mut self , f : F) -> & 'a mut SecurityControl < FrameCounterT , > where for < 'w > F : FnOnce (& 'w mut FrameCounterT) -> & 'w mut FrameCounterT { let mut cp = self . data . frame_counter ; self . data . frame_counter = * f (& mut cp) ; self . data } } impl < FrameCounterT , > SecurityControl < FrameCounterT , > where FrameCounterT : FrameCounter , { # [inline (always)]
pub fn new () -> Self { Self { security_control : 0 , frame_counter : FrameCounterT :: default () , } } pub fn security_control (& mut self) -> SecurityControl < FrameCounterT , > { SecurityControl :: new (self) } pub fn frame_counter (& mut self) -> FrameCounter < FrameCounterT , > { FrameCounter :: new (self) } } pub type SecurityControlDefault = SecurityControl < FrameCounterNone , > ;