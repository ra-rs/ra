#[doc = "Register `RFRH` reader"]
pub struct R(crate::R<RFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFRH` writer"]
pub struct W(crate::W<RFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFRH_SPEC>;
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
impl From<crate::W<RFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC16` reader - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type RFC16_R = crate::BitReader<bool>;
#[doc = "Field `RFC16` writer - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type RFC16_W<'a, const O: u8> = crate::BitWriter<'a, u16, RFRH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc16(&self) -> RFC16_R {
        RFC16_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn rfc16(&mut self) -> RFC16_W<0> {
        RFC16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfrh](index.html) module"]
pub struct RFRH_SPEC;
impl crate::RegisterSpec for RFRH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rfrh::R](R) reader structure"]
impl crate::Readable for RFRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfrh::W](W) writer structure"]
impl crate::Writable for RFRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFRH to value 0"]
impl crate::Resettable for RFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
