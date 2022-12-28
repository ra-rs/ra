#[doc = "Register `MALR` reader"]
pub struct R(crate::R<MALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MALR` writer"]
pub struct W(crate::W<MALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MALR_SPEC>;
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
impl From<crate::W<MALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MALR` reader - MAC Address Lower Bit RegisterThe MALR register sets the lower 16 bits of the 48-bit MAC address."]
pub type MALR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MALR` writer - MAC Address Lower Bit RegisterThe MALR register sets the lower 16 bits of the 48-bit MAC address."]
pub type MALR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MALR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - MAC Address Lower Bit RegisterThe MALR register sets the lower 16 bits of the 48-bit MAC address."]
    #[inline(always)]
    pub fn malr(&self) -> MALR_R {
        MALR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address Lower Bit RegisterThe MALR register sets the lower 16 bits of the 48-bit MAC address."]
    #[inline(always)]
    #[must_use]
    pub fn malr(&mut self) -> MALR_W<0> {
        MALR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address Lower Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [malr](index.html) module"]
pub struct MALR_SPEC;
impl crate::RegisterSpec for MALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [malr::R](R) reader structure"]
impl crate::Readable for MALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [malr::W](W) writer structure"]
impl crate::Writable for MALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MALR to value 0"]
impl crate::Resettable for MALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
