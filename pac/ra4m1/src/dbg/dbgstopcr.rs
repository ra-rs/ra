#[doc = "Register `DBGSTOPCR` reader"]
pub struct R(crate::R<DBGSTOPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGSTOPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGSTOPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGSTOPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGSTOPCR` writer"]
pub struct W(crate::W<DBGSTOPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGSTOPCR_SPEC>;
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
impl From<crate::W<DBGSTOPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGSTOPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGSTOP_IWDT` reader - Mask bit for IWDT reset/interrupt"]
pub type DBGSTOP_IWDT_R = crate::BitReader<DBGSTOP_IWDT_A>;
#[doc = "Mask bit for IWDT reset/interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_IWDT_A {
    #[doc = "0: Mask IWDT reset/interrupt"]
    _0 = 0,
    #[doc = "1: Enable IWDT reset"]
    _1 = 1,
}
impl From<DBGSTOP_IWDT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_IWDT_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_IWDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_IWDT_A {
        match self.bits {
            false => DBGSTOP_IWDT_A::_0,
            true => DBGSTOP_IWDT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_IWDT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_IWDT_A::_1
    }
}
#[doc = "Field `DBGSTOP_IWDT` writer - Mask bit for IWDT reset/interrupt"]
pub type DBGSTOP_IWDT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_IWDT_A, O>;
impl<'a, const O: u8> DBGSTOP_IWDT_W<'a, O> {
    #[doc = "Mask IWDT reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_IWDT_A::_0)
    }
    #[doc = "Enable IWDT reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_IWDT_A::_1)
    }
}
#[doc = "Field `DBGSTOP_WDT` reader - Mask bit for WDT reset/interrupt"]
pub type DBGSTOP_WDT_R = crate::BitReader<DBGSTOP_WDT_A>;
#[doc = "Mask bit for WDT reset/interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_WDT_A {
    #[doc = "0: Mask WDT reset/interrupt"]
    _0 = 0,
    #[doc = "1: Enable WDT reset"]
    _1 = 1,
}
impl From<DBGSTOP_WDT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_WDT_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_WDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_WDT_A {
        match self.bits {
            false => DBGSTOP_WDT_A::_0,
            true => DBGSTOP_WDT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_WDT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_WDT_A::_1
    }
}
#[doc = "Field `DBGSTOP_WDT` writer - Mask bit for WDT reset/interrupt"]
pub type DBGSTOP_WDT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_WDT_A, O>;
impl<'a, const O: u8> DBGSTOP_WDT_W<'a, O> {
    #[doc = "Mask WDT reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_WDT_A::_0)
    }
    #[doc = "Enable WDT reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_WDT_A::_1)
    }
}
#[doc = "Field `DBGSTOP_LVD` reader - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
pub type DBGSTOP_LVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBGSTOP_LVD` writer - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
pub type DBGSTOP_LVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBGSTOPCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBGSTOP_RPER` reader - Mask bit for RAM parity error reset/interrupt"]
pub type DBGSTOP_RPER_R = crate::BitReader<DBGSTOP_RPER_A>;
#[doc = "Mask bit for RAM parity error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_RPER_A {
    #[doc = "0: Enable RAM parity error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask RAM parity error reset/interrupt"]
    _1 = 1,
}
impl From<DBGSTOP_RPER_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_RPER_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_RPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_RPER_A {
        match self.bits {
            false => DBGSTOP_RPER_A::_0,
            true => DBGSTOP_RPER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_RPER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_RPER_A::_1
    }
}
#[doc = "Field `DBGSTOP_RPER` writer - Mask bit for RAM parity error reset/interrupt"]
pub type DBGSTOP_RPER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_RPER_A, O>;
impl<'a, const O: u8> DBGSTOP_RPER_W<'a, O> {
    #[doc = "Enable RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_RPER_A::_0)
    }
    #[doc = "Mask RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_RPER_A::_1)
    }
}
#[doc = "Field `DBGSTOP_RECCR` reader - Mask bit for RAM ECC error reset/interrupt"]
pub type DBGSTOP_RECCR_R = crate::BitReader<DBGSTOP_RECCR_A>;
#[doc = "Mask bit for RAM ECC error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_RECCR_A {
    #[doc = "0: Enable RAM ECC error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask RAM ECC error reset/interrupt"]
    _1 = 1,
}
impl From<DBGSTOP_RECCR_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_RECCR_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_RECCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_RECCR_A {
        match self.bits {
            false => DBGSTOP_RECCR_A::_0,
            true => DBGSTOP_RECCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_RECCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_RECCR_A::_1
    }
}
#[doc = "Field `DBGSTOP_RECCR` writer - Mask bit for RAM ECC error reset/interrupt"]
pub type DBGSTOP_RECCR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_RECCR_A, O>;
impl<'a, const O: u8> DBGSTOP_RECCR_W<'a, O> {
    #[doc = "Enable RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_RECCR_A::_0)
    }
    #[doc = "Mask RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_RECCR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mask bit for IWDT reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_iwdt(&self) -> DBGSTOP_IWDT_R {
        DBGSTOP_IWDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for WDT reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_wdt(&self) -> DBGSTOP_WDT_R {
        DBGSTOP_WDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:18 - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
    #[inline(always)]
    pub fn dbgstop_lvd(&self) -> DBGSTOP_LVD_R {
        DBGSTOP_LVD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Mask bit for RAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_rper(&self) -> DBGSTOP_RPER_R {
        DBGSTOP_RPER_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask bit for RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_reccr(&self) -> DBGSTOP_RECCR_R {
        DBGSTOP_RECCR_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for IWDT reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_iwdt(&mut self) -> DBGSTOP_IWDT_W<0> {
        DBGSTOP_IWDT_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for WDT reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_wdt(&mut self) -> DBGSTOP_WDT_W<1> {
        DBGSTOP_WDT_W::new(self)
    }
    #[doc = "Bits 16:18 - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_lvd(&mut self) -> DBGSTOP_LVD_W<16> {
        DBGSTOP_LVD_W::new(self)
    }
    #[doc = "Bit 24 - Mask bit for RAM parity error reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_rper(&mut self) -> DBGSTOP_RPER_W<24> {
        DBGSTOP_RPER_W::new(self)
    }
    #[doc = "Bit 25 - Mask bit for RAM ECC error reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_reccr(&mut self) -> DBGSTOP_RECCR_W<25> {
        DBGSTOP_RECCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Stop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgstopcr](index.html) module"]
pub struct DBGSTOPCR_SPEC;
impl crate::RegisterSpec for DBGSTOPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgstopcr::R](R) reader structure"]
impl crate::Readable for DBGSTOPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgstopcr::W](W) writer structure"]
impl crate::Writable for DBGSTOPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGSTOPCR to value 0x03"]
impl crate::Resettable for DBGSTOPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
