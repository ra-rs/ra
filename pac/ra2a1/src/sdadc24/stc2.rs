#[doc = "Register `STC2` reader"]
pub struct R(crate::R<STC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STC2` writer"]
pub struct W(crate::W<STC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STC2_SPEC>;
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
impl From<crate::W<STC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGRPON` reader - BGR part power control"]
pub type BGRPON_R = crate::BitReader<BGRPON_A>;
#[doc = "BGR part power control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGRPON_A {
    #[doc = "0: Turn off the power of ADBGR, SBIAS/VREFI, and ADREG"]
    _0 = 0,
    #[doc = "1: Turn on the power of ADBGR, SBIAS/VREFI, and ADREG"]
    _1 = 1,
}
impl From<BGRPON_A> for bool {
    #[inline(always)]
    fn from(variant: BGRPON_A) -> Self {
        variant as u8 != 0
    }
}
impl BGRPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGRPON_A {
        match self.bits {
            false => BGRPON_A::_0,
            true => BGRPON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGRPON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGRPON_A::_1
    }
}
#[doc = "Field `BGRPON` writer - BGR part power control"]
pub type BGRPON_W<'a, const O: u8> = crate::BitWriter<'a, u8, STC2_SPEC, BGRPON_A, O>;
impl<'a, const O: u8> BGRPON_W<'a, O> {
    #[doc = "Turn off the power of ADBGR, SBIAS/VREFI, and ADREG"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGRPON_A::_0)
    }
    #[doc = "Turn on the power of ADBGR, SBIAS/VREFI, and ADREG"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGRPON_A::_1)
    }
}
#[doc = "Field `ADCPON` reader - ADC reference supply part power control"]
pub type ADCPON_R = crate::BitReader<ADCPON_A>;
#[doc = "ADC reference supply part power control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPON_A {
    #[doc = "0: Turn off the power of VBIAS, PGA and sigma-delta A/D converter"]
    _0 = 0,
    #[doc = "1: Turn on the power of VBIAS, PGA and sigma-delta A/D converter"]
    _1 = 1,
}
impl From<ADCPON_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPON_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPON_A {
        match self.bits {
            false => ADCPON_A::_0,
            true => ADCPON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADCPON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADCPON_A::_1
    }
}
#[doc = "Field `ADCPON` writer - ADC reference supply part power control"]
pub type ADCPON_W<'a, const O: u8> = crate::BitWriter<'a, u8, STC2_SPEC, ADCPON_A, O>;
impl<'a, const O: u8> ADCPON_W<'a, O> {
    #[doc = "Turn off the power of VBIAS, PGA and sigma-delta A/D converter"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCPON_A::_0)
    }
    #[doc = "Turn on the power of VBIAS, PGA and sigma-delta A/D converter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCPON_A::_1)
    }
}
#[doc = "Field `ADFPWDS` reader - ADREG forced power-down mode"]
pub type ADFPWDS_R = crate::BitReader<ADFPWDS_A>;
#[doc = "ADREG forced power-down mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADFPWDS_A {
    #[doc = "0: Power of ADREG is controlled by the BGRPON setting"]
    _0 = 0,
    #[doc = "1: Power of only ADREG is turned off regardless of the BGRPON setting"]
    _1 = 1,
}
impl From<ADFPWDS_A> for bool {
    #[inline(always)]
    fn from(variant: ADFPWDS_A) -> Self {
        variant as u8 != 0
    }
}
impl ADFPWDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADFPWDS_A {
        match self.bits {
            false => ADFPWDS_A::_0,
            true => ADFPWDS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADFPWDS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADFPWDS_A::_1
    }
}
#[doc = "Field `ADFPWDS` writer - ADREG forced power-down mode"]
pub type ADFPWDS_W<'a, const O: u8> = crate::BitWriter<'a, u8, STC2_SPEC, ADFPWDS_A, O>;
impl<'a, const O: u8> ADFPWDS_W<'a, O> {
    #[doc = "Power of ADREG is controlled by the BGRPON setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADFPWDS_A::_0)
    }
    #[doc = "Power of only ADREG is turned off regardless of the BGRPON setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADFPWDS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BGR part power control"]
    #[inline(always)]
    pub fn bgrpon(&self) -> BGRPON_R {
        BGRPON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC reference supply part power control"]
    #[inline(always)]
    pub fn adcpon(&self) -> ADCPON_R {
        ADCPON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADREG forced power-down mode"]
    #[inline(always)]
    pub fn adfpwds(&self) -> ADFPWDS_R {
        ADFPWDS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BGR part power control"]
    #[inline(always)]
    #[must_use]
    pub fn bgrpon(&mut self) -> BGRPON_W<0> {
        BGRPON_W::new(self)
    }
    #[doc = "Bit 1 - ADC reference supply part power control"]
    #[inline(always)]
    #[must_use]
    pub fn adcpon(&mut self) -> ADCPON_W<1> {
        ADCPON_W::new(self)
    }
    #[doc = "Bit 2 - ADREG forced power-down mode"]
    #[inline(always)]
    #[must_use]
    pub fn adfpwds(&mut self) -> ADFPWDS_W<2> {
        ADFPWDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Startup Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stc2](index.html) module"]
pub struct STC2_SPEC;
impl crate::RegisterSpec for STC2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [stc2::R](R) reader structure"]
impl crate::Readable for STC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stc2::W](W) writer structure"]
impl crate::Writable for STC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STC2 to value 0"]
impl crate::Resettable for STC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
