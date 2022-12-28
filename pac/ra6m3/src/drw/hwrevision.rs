#[doc = "Register `HWREVISION` reader"]
pub struct R(crate::R<HWREVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWREVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWREVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWREVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REV` reader - Revision number"]
pub type REV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DLR` reader - Display list reader feature"]
pub type DLR_R = crate::BitReader<DLR_A>;
#[doc = "Display list reader feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLR_A {
    #[doc = "0: Display list reader unavailable"]
    _0 = 0,
    #[doc = "1: Display list reader available"]
    _1 = 1,
}
impl From<DLR_A> for bool {
    #[inline(always)]
    fn from(variant: DLR_A) -> Self {
        variant as u8 != 0
    }
}
impl DLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLR_A {
        match self.bits {
            false => DLR_A::_0,
            true => DLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLR_A::_1
    }
}
#[doc = "Field `FBCACHE` reader - Framebuffer cache feature"]
pub type FBCACHE_R = crate::BitReader<FBCACHE_A>;
#[doc = "Framebuffer cache feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBCACHE_A {
    #[doc = "0: Framebuffer cache unavailable"]
    _0 = 0,
    #[doc = "1: Framebuffer cache available"]
    _1 = 1,
}
impl From<FBCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: FBCACHE_A) -> Self {
        variant as u8 != 0
    }
}
impl FBCACHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBCACHE_A {
        match self.bits {
            false => FBCACHE_A::_0,
            true => FBCACHE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FBCACHE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FBCACHE_A::_1
    }
}
#[doc = "Field `TXCACHE` reader - Texture cache feature"]
pub type TXCACHE_R = crate::BitReader<TXCACHE_A>;
#[doc = "Texture cache feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXCACHE_A {
    #[doc = "0: Texture cache unavailable"]
    _0 = 0,
    #[doc = "1: Texture cache available"]
    _1 = 1,
}
impl From<TXCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: TXCACHE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXCACHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCACHE_A {
        match self.bits {
            false => TXCACHE_A::_0,
            true => TXCACHE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXCACHE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXCACHE_A::_1
    }
}
#[doc = "Field `PERFCOUNT` reader - Two performance counter feature"]
pub type PERFCOUNT_R = crate::BitReader<PERFCOUNT_A>;
#[doc = "Two performance counter feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERFCOUNT_A {
    #[doc = "0: Two performance counter unavailable"]
    _0 = 0,
    #[doc = "1: Two performance counter available"]
    _1 = 1,
}
impl From<PERFCOUNT_A> for bool {
    #[inline(always)]
    fn from(variant: PERFCOUNT_A) -> Self {
        variant as u8 != 0
    }
}
impl PERFCOUNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERFCOUNT_A {
        match self.bits {
            false => PERFCOUNT_A::_0,
            true => PERFCOUNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PERFCOUNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PERFCOUNT_A::_1
    }
}
#[doc = "Field `TEXCLU` reader - Texture CLUT with 16 or 256 entries feature"]
pub type TEXCLU_R = crate::BitReader<TEXCLU_A>;
#[doc = "Texture CLUT with 16 or 256 entries feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXCLU_A {
    #[doc = "0: Texture CLUT with 16 or 256 entries unavailable"]
    _0 = 0,
    #[doc = "1: Texture CLUT with 16 or 256 entries available"]
    _1 = 1,
}
impl From<TEXCLU_A> for bool {
    #[inline(always)]
    fn from(variant: TEXCLU_A) -> Self {
        variant as u8 != 0
    }
}
impl TEXCLU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEXCLU_A {
        match self.bits {
            false => TEXCLU_A::_0,
            true => TEXCLU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEXCLU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEXCLU_A::_1
    }
}
#[doc = "Field `RLEUNIT` reader - RLE unit feature"]
pub type RLEUNIT_R = crate::BitReader<RLEUNIT_A>;
#[doc = "RLE unit feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLEUNIT_A {
    #[doc = "0: RLE unit unavailable"]
    _0 = 0,
    #[doc = "1: RLE unit available"]
    _1 = 1,
}
impl From<RLEUNIT_A> for bool {
    #[inline(always)]
    fn from(variant: RLEUNIT_A) -> Self {
        variant as u8 != 0
    }
}
impl RLEUNIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLEUNIT_A {
        match self.bits {
            false => RLEUNIT_A::_0,
            true => RLEUNIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RLEUNIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RLEUNIT_A::_1
    }
}
#[doc = "Field `TEXCLUT256` reader - Texture CLUT feature"]
pub type TEXCLUT256_R = crate::BitReader<TEXCLUT256_A>;
#[doc = "Texture CLUT feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXCLUT256_A {
    #[doc = "0: Texture CLUT unavailable"]
    _0 = 0,
    #[doc = "1: Texture CLUT available"]
    _1 = 1,
}
impl From<TEXCLUT256_A> for bool {
    #[inline(always)]
    fn from(variant: TEXCLUT256_A) -> Self {
        variant as u8 != 0
    }
}
impl TEXCLUT256_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEXCLUT256_A {
        match self.bits {
            false => TEXCLUT256_A::_0,
            true => TEXCLUT256_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEXCLUT256_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEXCLUT256_A::_1
    }
}
#[doc = "Field `COLORKEY` reader - Colorkey feature"]
pub type COLORKEY_R = crate::BitReader<COLORKEY_A>;
#[doc = "Colorkey feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLORKEY_A {
    #[doc = "0: Colorkey unavailable"]
    _0 = 0,
    #[doc = "1: Colorkey available"]
    _1 = 1,
}
impl From<COLORKEY_A> for bool {
    #[inline(always)]
    fn from(variant: COLORKEY_A) -> Self {
        variant as u8 != 0
    }
}
impl COLORKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLORKEY_A {
        match self.bits {
            false => COLORKEY_A::_0,
            true => COLORKEY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COLORKEY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COLORKEY_A::_1
    }
}
#[doc = "Field `ACBLEND` reader - Alpha channel blending feature"]
pub type ACBLEND_R = crate::BitReader<ACBLEND_A>;
#[doc = "Alpha channel blending feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACBLEND_A {
    #[doc = "0: Alpha channel blending unavailable"]
    _0 = 0,
    #[doc = "1: Alpha channel blending available"]
    _1 = 1,
}
impl From<ACBLEND_A> for bool {
    #[inline(always)]
    fn from(variant: ACBLEND_A) -> Self {
        variant as u8 != 0
    }
}
impl ACBLEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACBLEND_A {
        match self.bits {
            false => ACBLEND_A::_0,
            true => ACBLEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACBLEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACBLEND_A::_1
    }
}
impl R {
    #[doc = "Bits 0:11 - Revision number"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 17 - Display list reader feature"]
    #[inline(always)]
    pub fn dlr(&self) -> DLR_R {
        DLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Framebuffer cache feature"]
    #[inline(always)]
    pub fn fbcache(&self) -> FBCACHE_R {
        FBCACHE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Texture cache feature"]
    #[inline(always)]
    pub fn txcache(&self) -> TXCACHE_R {
        TXCACHE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Two performance counter feature"]
    #[inline(always)]
    pub fn perfcount(&self) -> PERFCOUNT_R {
        PERFCOUNT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Texture CLUT with 16 or 256 entries feature"]
    #[inline(always)]
    pub fn texclu(&self) -> TEXCLU_R {
        TEXCLU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - RLE unit feature"]
    #[inline(always)]
    pub fn rleunit(&self) -> RLEUNIT_R {
        RLEUNIT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Texture CLUT feature"]
    #[inline(always)]
    pub fn texclut256(&self) -> TEXCLUT256_R {
        TEXCLUT256_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Colorkey feature"]
    #[inline(always)]
    pub fn colorkey(&self) -> COLORKEY_R {
        COLORKEY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Alpha channel blending feature"]
    #[inline(always)]
    pub fn acblend(&self) -> ACBLEND_R {
        ACBLEND_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Hardware Version and Feature Set ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwrevision](index.html) module"]
pub struct HWREVISION_SPEC;
impl crate::RegisterSpec for HWREVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwrevision::R](R) reader structure"]
impl crate::Readable for HWREVISION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWREVISION to value 0x0fbe_0107"]
impl crate::Resettable for HWREVISION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fbe_0107;
}
