#[doc = "Register `ADCMPBSR` reader"]
pub struct R(crate::R<ADCMPBSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPBSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPBSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPBSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPBSR` writer"]
pub struct W(crate::W<ADCMPBSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPBSR_SPEC>;
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
impl From<crate::W<ADCMPBSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPBSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSTB` reader - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN003, AN005-AN007, AN016-AN018, AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition.\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTB_R = crate::BitReader<CMPSTB_A>;
#[doc = "Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN003, AN005-AN007, AN016-AN018, AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTB_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTB_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTB_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTB_A {
        match self.bits {
            false => CMPSTB_A::_0,
            true => CMPSTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTB_A::_1
    }
}
#[doc = "Field `CMPSTB` writer - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN003, AN005-AN007, AN016-AN018, AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
pub type CMPSTB_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ADCMPBSR_SPEC, CMPSTB_A, O>;
impl<'a, const O: u8> CMPSTB_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTB_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTB_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN003, AN005-AN007, AN016-AN018, AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
    #[inline(always)]
    pub fn cmpstb(&self) -> CMPSTB_R {
        CMPSTB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN003, AN005-AN007, AN016-AN018, AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
    #[inline(always)]
    #[must_use]
    pub fn cmpstb(&mut self) -> CMPSTB_W<0> {
        CMPSTB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window B Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpbsr](index.html) module"]
pub struct ADCMPBSR_SPEC;
impl crate::RegisterSpec for ADCMPBSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcmpbsr::R](R) reader structure"]
impl crate::Readable for ADCMPBSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpbsr::W](W) writer structure"]
impl crate::Writable for ADCMPBSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPBSR to value 0"]
impl crate::Resettable for ADCMPBSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
