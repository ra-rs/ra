#[doc = "Register `SD_INFO1_MASK` reader"]
pub struct R(crate::R<SD_INFO1_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_INFO1_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_INFO1_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_INFO1_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_INFO1_MASK` writer"]
pub struct W(crate::W<SD_INFO1_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_INFO1_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SD_INFO1_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_INFO1_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSPENDM` reader - Response End Interrupt Request Mask"]
pub type RSPENDM_R = crate::BitReader<RSPENDM_A>;
#[doc = "Response End Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPENDM_A {
    #[doc = "0: Do not mask response end interrupt request"]
    _0 = 0,
    #[doc = "1: Mask response end interrupt request"]
    _1 = 1,
}
impl From<RSPENDM_A> for bool {
    #[inline(always)]
    fn from(variant: RSPENDM_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPENDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPENDM_A {
        match self.bits {
            false => RSPENDM_A::_0,
            true => RSPENDM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPENDM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPENDM_A::_1
    }
}
#[doc = "Field `RSPENDM` writer - Response End Interrupt Request Mask"]
pub type RSPENDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO1_MASK_SPEC, RSPENDM_A, O>;
impl<'a, const O: u8> RSPENDM_W<'a, O> {
    #[doc = "Do not mask response end interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPENDM_A::_0)
    }
    #[doc = "Mask response end interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPENDM_A::_1)
    }
}
#[doc = "Field `ACENDM` reader - Access End Interrupt Request Mask"]
pub type ACENDM_R = crate::BitReader<ACENDM_A>;
#[doc = "Access End Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACENDM_A {
    #[doc = "0: Do not mask access end interrupt request"]
    _0 = 0,
    #[doc = "1: Mask access end interrupt request"]
    _1 = 1,
}
impl From<ACENDM_A> for bool {
    #[inline(always)]
    fn from(variant: ACENDM_A) -> Self {
        variant as u8 != 0
    }
}
impl ACENDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACENDM_A {
        match self.bits {
            false => ACENDM_A::_0,
            true => ACENDM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACENDM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACENDM_A::_1
    }
}
#[doc = "Field `ACENDM` writer - Access End Interrupt Request Mask"]
pub type ACENDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO1_MASK_SPEC, ACENDM_A, O>;
impl<'a, const O: u8> ACENDM_W<'a, O> {
    #[doc = "Do not mask access end interrupt request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACENDM_A::_0)
    }
    #[doc = "Mask access end interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACENDM_A::_1)
    }
}
#[doc = "Field `SDCDRMM` reader - SDnCD Removal Interrupt Request Mask"]
pub type SDCDRMM_R = crate::BitReader<SDCDRMM_A>;
#[doc = "SDnCD Removal Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDRMM_A {
    #[doc = "0: Do not mask SD card/MMC removal interrupt request by the SDnCD pin"]
    _0 = 0,
    #[doc = "1: Mask SD card/MMC removal interrupt request by the SDnCD pin"]
    _1 = 1,
}
impl From<SDCDRMM_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDRMM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDCDRMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCDRMM_A {
        match self.bits {
            false => SDCDRMM_A::_0,
            true => SDCDRMM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDRMM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDRMM_A::_1
    }
}
#[doc = "Field `SDCDRMM` writer - SDnCD Removal Interrupt Request Mask"]
pub type SDCDRMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO1_MASK_SPEC, SDCDRMM_A, O>;
impl<'a, const O: u8> SDCDRMM_W<'a, O> {
    #[doc = "Do not mask SD card/MMC removal interrupt request by the SDnCD pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDCDRMM_A::_0)
    }
    #[doc = "Mask SD card/MMC removal interrupt request by the SDnCD pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCDRMM_A::_1)
    }
}
#[doc = "Field `SDCDINM` reader - SDnCD Insertion Interrupt Request Mask"]
pub type SDCDINM_R = crate::BitReader<SDCDINM_A>;
#[doc = "SDnCD Insertion Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDINM_A {
    #[doc = "0: Do not mask SD card/MMC insertion interrupt request by the SDnCD pin"]
    _0 = 0,
    #[doc = "1: Mask SD card/MMC insertion interrupt request by the SDnCD pin"]
    _1 = 1,
}
impl From<SDCDINM_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDINM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDCDINM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCDINM_A {
        match self.bits {
            false => SDCDINM_A::_0,
            true => SDCDINM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDINM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDINM_A::_1
    }
}
#[doc = "Field `SDCDINM` writer - SDnCD Insertion Interrupt Request Mask"]
pub type SDCDINM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO1_MASK_SPEC, SDCDINM_A, O>;
impl<'a, const O: u8> SDCDINM_W<'a, O> {
    #[doc = "Do not mask SD card/MMC insertion interrupt request by the SDnCD pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDCDINM_A::_0)
    }
    #[doc = "Mask SD card/MMC insertion interrupt request by the SDnCD pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCDINM_A::_1)
    }
}
#[doc = "Field `SDD3RMM` reader - SDnDAT3 Removal Interrupt Request Mask"]
pub type SDD3RMM_R = crate::BitReader<SDD3RMM_A>;
#[doc = "SDnDAT3 Removal Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3RMM_A {
    #[doc = "0: Do not mask SD card/MMC removal interrupt request by the SDnDAT3 pin"]
    _0 = 0,
    #[doc = "1: Mask SD card/MMC removal interrupt request by the SDnDAT3 pin"]
    _1 = 1,
}
impl From<SDD3RMM_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3RMM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDD3RMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDD3RMM_A {
        match self.bits {
            false => SDD3RMM_A::_0,
            true => SDD3RMM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3RMM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3RMM_A::_1
    }
}
#[doc = "Field `SDD3RMM` writer - SDnDAT3 Removal Interrupt Request Mask"]
pub type SDD3RMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO1_MASK_SPEC, SDD3RMM_A, O>;
impl<'a, const O: u8> SDD3RMM_W<'a, O> {
    #[doc = "Do not mask SD card/MMC removal interrupt request by the SDnDAT3 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDD3RMM_A::_0)
    }
    #[doc = "Mask SD card/MMC removal interrupt request by the SDnDAT3 pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDD3RMM_A::_1)
    }
}
#[doc = "Field `SDD3INM` reader - SDnDAT3 Insertion Interrupt Request Mask"]
pub type SDD3INM_R = crate::BitReader<SDD3INM_A>;
#[doc = "SDnDAT3 Insertion Interrupt Request Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3INM_A {
    #[doc = "0: Do not mask SD card/MMC insertion interrupt request by the SDnDAT3 pin"]
    _0 = 0,
    #[doc = "1: Mask SD card/MMC insertion interrupt request by the SDnDAT3 pin"]
    _1 = 1,
}
impl From<SDD3INM_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3INM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDD3INM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDD3INM_A {
        match self.bits {
            false => SDD3INM_A::_0,
            true => SDD3INM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3INM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3INM_A::_1
    }
}
#[doc = "Field `SDD3INM` writer - SDnDAT3 Insertion Interrupt Request Mask"]
pub type SDD3INM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_INFO1_MASK_SPEC, SDD3INM_A, O>;
impl<'a, const O: u8> SDD3INM_W<'a, O> {
    #[doc = "Do not mask SD card/MMC insertion interrupt request by the SDnDAT3 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDD3INM_A::_0)
    }
    #[doc = "Mask SD card/MMC insertion interrupt request by the SDnDAT3 pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDD3INM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Response End Interrupt Request Mask"]
    #[inline(always)]
    pub fn rspendm(&self) -> RSPENDM_R {
        RSPENDM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Access End Interrupt Request Mask"]
    #[inline(always)]
    pub fn acendm(&self) -> ACENDM_R {
        ACENDM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDnCD Removal Interrupt Request Mask"]
    #[inline(always)]
    pub fn sdcdrmm(&self) -> SDCDRMM_R {
        SDCDRMM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDnCD Insertion Interrupt Request Mask"]
    #[inline(always)]
    pub fn sdcdinm(&self) -> SDCDINM_R {
        SDCDINM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SDnDAT3 Removal Interrupt Request Mask"]
    #[inline(always)]
    pub fn sdd3rmm(&self) -> SDD3RMM_R {
        SDD3RMM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDnDAT3 Insertion Interrupt Request Mask"]
    #[inline(always)]
    pub fn sdd3inm(&self) -> SDD3INM_R {
        SDD3INM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Response End Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rspendm(&mut self) -> RSPENDM_W<0> {
        RSPENDM_W::new(self)
    }
    #[doc = "Bit 2 - Access End Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn acendm(&mut self) -> ACENDM_W<2> {
        ACENDM_W::new(self)
    }
    #[doc = "Bit 3 - SDnCD Removal Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdcdrmm(&mut self) -> SDCDRMM_W<3> {
        SDCDRMM_W::new(self)
    }
    #[doc = "Bit 4 - SDnCD Insertion Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdcdinm(&mut self) -> SDCDINM_W<4> {
        SDCDINM_W::new(self)
    }
    #[doc = "Bit 8 - SDnDAT3 Removal Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdd3rmm(&mut self) -> SDD3RMM_W<8> {
        SDD3RMM_W::new(self)
    }
    #[doc = "Bit 9 - SDnDAT3 Insertion Interrupt Request Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdd3inm(&mut self) -> SDD3INM_W<9> {
        SDD3INM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD INFO1 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_info1_mask](index.html) module"]
pub struct SD_INFO1_MASK_SPEC;
impl crate::RegisterSpec for SD_INFO1_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_info1_mask::R](R) reader structure"]
impl crate::Readable for SD_INFO1_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_info1_mask::W](W) writer structure"]
impl crate::Writable for SD_INFO1_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_INFO1_MASK to value 0x031d"]
impl crate::Resettable for SD_INFO1_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x031d;
}
