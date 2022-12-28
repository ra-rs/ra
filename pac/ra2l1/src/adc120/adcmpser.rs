#[doc = "Register `ADCMPSER` reader"]
pub struct R(crate::R<ADCMPSER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPSER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPSER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPSER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPSER` writer"]
pub struct W(crate::W<ADCMPSER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPSER_SPEC>;
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
impl From<crate::W<ADCMPSER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPSER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSTTSA` reader - Compare Window A Temperature Sensor Output Compare Flag"]
pub type CMPSTTSA_R = crate::BitReader<CMPSTTSA_A>;
#[doc = "Compare Window A Temperature Sensor Output Compare Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTTSA_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTTSA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTTSA_A {
        match self.bits {
            false => CMPSTTSA_A::_0,
            true => CMPSTTSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTTSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTTSA_A::_1
    }
}
#[doc = "Field `CMPSTTSA` writer - Compare Window A Temperature Sensor Output Compare Flag"]
pub type CMPSTTSA_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCMPSER_SPEC, CMPSTTSA_A, O>;
impl<'a, const O: u8> CMPSTTSA_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTTSA_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTTSA_A::_1)
    }
}
#[doc = "Field `CMPSTOCA` reader - Compare Window A Internal Reference Voltage Compare Flag"]
pub type CMPSTOCA_R = crate::BitReader<CMPSTOCA_A>;
#[doc = "Compare Window A Internal Reference Voltage Compare Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTOCA_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTOCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTOCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTOCA_A {
        match self.bits {
            false => CMPSTOCA_A::_0,
            true => CMPSTOCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTOCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTOCA_A::_1
    }
}
#[doc = "Field `CMPSTOCA` writer - Compare Window A Internal Reference Voltage Compare Flag"]
pub type CMPSTOCA_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCMPSER_SPEC, CMPSTOCA_A, O>;
impl<'a, const O: u8> CMPSTOCA_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTOCA_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTOCA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Compare Flag"]
    #[inline(always)]
    pub fn cmpsttsa(&self) -> CMPSTTSA_R {
        CMPSTTSA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Compare Flag"]
    #[inline(always)]
    pub fn cmpstoca(&self) -> CMPSTOCA_R {
        CMPSTOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Compare Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsttsa(&mut self) -> CMPSTTSA_W<0> {
        CMPSTTSA_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Compare Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstoca(&mut self) -> CMPSTOCA_W<1> {
        CMPSTOCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Extended Input Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpser](index.html) module"]
pub struct ADCMPSER_SPEC;
impl crate::RegisterSpec for ADCMPSER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcmpser::R](R) reader structure"]
impl crate::Readable for ADCMPSER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpser::W](W) writer structure"]
impl crate::Writable for ADCMPSER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPSER to value 0"]
impl crate::Resettable for ADCMPSER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
