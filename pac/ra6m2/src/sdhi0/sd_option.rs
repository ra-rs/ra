#[doc = "Register `SD_OPTION` reader"]
pub struct R(crate::R<SD_OPTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_OPTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_OPTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_OPTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_OPTION` writer"]
pub struct W(crate::W<SD_OPTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_OPTION_SPEC>;
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
impl From<crate::W<SD_OPTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_OPTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTOP` reader - Card Detect Time Counter"]
pub type CTOP_R = crate::FieldReader<u8, CTOP_A>;
#[doc = "Card Detect Time Counter\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTOP_A {
    #[doc = "15: Setting prohibited"]
    _1111 = 15,
}
impl From<CTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: CTOP_A) -> Self {
        variant as _
    }
}
impl CTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTOP_A> {
        match self.bits {
            15 => Some(CTOP_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CTOP_A::_1111
    }
}
#[doc = "Field `CTOP` writer - Card Detect Time Counter"]
pub type CTOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_OPTION_SPEC, u8, CTOP_A, 4, O>;
impl<'a, const O: u8> CTOP_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(CTOP_A::_1111)
    }
}
#[doc = "Field `TOP` reader - Timeout Counter"]
pub type TOP_R = crate::FieldReader<u8, TOP_A>;
#[doc = "Timeout Counter\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOP_A {
    #[doc = "15: Setting prohibited"]
    _1111 = 15,
}
impl From<TOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TOP_A) -> Self {
        variant as _
    }
}
impl TOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOP_A> {
        match self.bits {
            15 => Some(TOP_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TOP_A::_1111
    }
}
#[doc = "Field `TOP` writer - Timeout Counter"]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_OPTION_SPEC, u8, TOP_A, 4, O>;
impl<'a, const O: u8> TOP_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TOP_A::_1111)
    }
}
#[doc = "Field `TOUTMASK` reader - Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence."]
pub type TOUTMASK_R = crate::BitReader<TOUTMASK_A>;
#[doc = "Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOUTMASK_A {
    #[doc = "0: Activate Timeout"]
    _0 = 0,
    #[doc = "1: Inactivate Timeout(RSPTO bit and DTO bit of SD_INFO2 and SD_ERR_STS2 won't be set)"]
    _1 = 1,
}
impl From<TOUTMASK_A> for bool {
    #[inline(always)]
    fn from(variant: TOUTMASK_A) -> Self {
        variant as u8 != 0
    }
}
impl TOUTMASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTMASK_A {
        match self.bits {
            false => TOUTMASK_A::_0,
            true => TOUTMASK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOUTMASK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOUTMASK_A::_1
    }
}
#[doc = "Field `TOUTMASK` writer - Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence."]
pub type TOUTMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_OPTION_SPEC, TOUTMASK_A, O>;
impl<'a, const O: u8> TOUTMASK_W<'a, O> {
    #[doc = "Activate Timeout"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTMASK_A::_0)
    }
    #[doc = "Inactivate Timeout(RSPTO bit and DTO bit of SD_INFO2 and SD_ERR_STS2 won't be set)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTMASK_A::_1)
    }
}
#[doc = "Field `WIDTH8` reader - Bus Widthsee b15, WIDTH bit"]
pub type WIDTH8_R = crate::BitReader<bool>;
#[doc = "Field `WIDTH8` writer - Bus Widthsee b15, WIDTH bit"]
pub type WIDTH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_OPTION_SPEC, bool, O>;
#[doc = "Field `WIDTH` reader - Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0."]
pub type WIDTH_R = crate::BitReader<WIDTH_A>;
#[doc = "Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIDTH_A {
    #[doc = "0: 4-bit width (WIDTH8=0) / 8-bit width (WIDTH8=1)"]
    _0 = 0,
    #[doc = "1: 1-bit width (WIDTH8=0 or 1 )"]
    _1 = 1,
}
impl From<WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as u8 != 0
    }
}
impl WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIDTH_A {
        match self.bits {
            false => WIDTH_A::_0,
            true => WIDTH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WIDTH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WIDTH_A::_1
    }
}
#[doc = "Field `WIDTH` writer - Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0."]
pub type WIDTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_OPTION_SPEC, WIDTH_A, O>;
impl<'a, const O: u8> WIDTH_W<'a, O> {
    #[doc = "4-bit width (WIDTH8=0) / 8-bit width (WIDTH8=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WIDTH_A::_0)
    }
    #[doc = "1-bit width (WIDTH8=0 or 1 )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WIDTH_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Card Detect Time Counter"]
    #[inline(always)]
    pub fn ctop(&self) -> CTOP_R {
        CTOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Timeout Counter"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence."]
    #[inline(always)]
    pub fn toutmask(&self) -> TOUTMASK_R {
        TOUTMASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Bus Widthsee b15, WIDTH bit"]
    #[inline(always)]
    pub fn width8(&self) -> WIDTH8_R {
        WIDTH8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Card Detect Time Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ctop(&mut self) -> CTOP_W<0> {
        CTOP_W::new(self)
    }
    #[doc = "Bits 4:7 - Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<4> {
        TOP_W::new(self)
    }
    #[doc = "Bit 8 - Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence."]
    #[inline(always)]
    #[must_use]
    pub fn toutmask(&mut self) -> TOUTMASK_W<8> {
        TOUTMASK_W::new(self)
    }
    #[doc = "Bit 13 - Bus Widthsee b15, WIDTH bit"]
    #[inline(always)]
    #[must_use]
    pub fn width8(&mut self) -> WIDTH8_W<13> {
        WIDTH8_W::new(self)
    }
    #[doc = "Bit 15 - Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<15> {
        WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Card Access Control Option Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_option](index.html) module"]
pub struct SD_OPTION_SPEC;
impl crate::RegisterSpec for SD_OPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_option::R](R) reader structure"]
impl crate::Readable for SD_OPTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_option::W](W) writer structure"]
impl crate::Writable for SD_OPTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_OPTION to value 0x40ee"]
impl crate::Resettable for SD_OPTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x40ee;
}
