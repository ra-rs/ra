#[doc = "Register `TSCR` reader"]
pub struct R(crate::R<TSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSCR` writer"]
pub struct W(crate::W<TSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCR_SPEC>;
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
impl From<crate::W<TSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSOE` reader - Temperature Sensor Output Enable"]
pub type TSOE_R = crate::BitReader<TSOE_A>;
#[doc = "Temperature Sensor Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOE_A {
    #[doc = "0: Disable output from the temperature sensor to the ADC"]
    _0 = 0,
    #[doc = "1: Enable output from the temperature sensor to the ADC"]
    _1 = 1,
}
impl From<TSOE_A> for bool {
    #[inline(always)]
    fn from(variant: TSOE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSOE_A {
        match self.bits {
            false => TSOE_A::_0,
            true => TSOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSOE_A::_1
    }
}
#[doc = "Field `TSOE` writer - Temperature Sensor Output Enable"]
pub type TSOE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TSCR_SPEC, TSOE_A, O>;
impl<'a, const O: u8> TSOE_W<'a, O> {
    #[doc = "Disable output from the temperature sensor to the ADC"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSOE_A::_0)
    }
    #[doc = "Enable output from the temperature sensor to the ADC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSOE_A::_1)
    }
}
#[doc = "Field `TSEN` reader - Temperature Sensor Enable"]
pub type TSEN_R = crate::BitReader<TSEN_A>;
#[doc = "Temperature Sensor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN_A {
    #[doc = "0: Stop the temperature sensor"]
    _0 = 0,
    #[doc = "1: Start the temperature sensor."]
    _1 = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::_0,
            true => TSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSEN_A::_1
    }
}
#[doc = "Field `TSEN` writer - Temperature Sensor Enable"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, TSCR_SPEC, TSEN_A, O>;
impl<'a, const O: u8> TSEN_W<'a, O> {
    #[doc = "Stop the temperature sensor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSEN_A::_0)
    }
    #[doc = "Start the temperature sensor."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsoe(&self) -> TSOE_R {
        TSOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Temperature Sensor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsoe(&mut self) -> TSOE_W<4> {
        TSOE_W::new(self)
    }
    #[doc = "Bit 7 - Temperature Sensor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<7> {
        TSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature Sensor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscr](index.html) module"]
pub struct TSCR_SPEC;
impl crate::RegisterSpec for TSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tscr::R](R) reader structure"]
impl crate::Readable for TSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tscr::W](W) writer structure"]
impl crate::Writable for TSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSCR to value 0"]
impl crate::Resettable for TSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
