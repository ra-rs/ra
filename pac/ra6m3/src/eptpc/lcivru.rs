#[doc = "Register `LCIVRU` reader"]
pub struct R(crate::R<LCIVRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCIVRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCIVRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCIVRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCIVRU` writer"]
pub struct W(crate::W<LCIVRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCIVRU_SPEC>;
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
impl From<crate::W<LCIVRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCIVRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCIVRU` reader - These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter."]
pub type LCIVRU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LCIVRU` writer - These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter."]
pub type LCIVRU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCIVRU_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter."]
    #[inline(always)]
    pub fn lcivru(&self) -> LCIVRU_R {
        LCIVRU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter."]
    #[inline(always)]
    #[must_use]
    pub fn lcivru(&mut self) -> LCIVRU_W<0> {
        LCIVRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local Time Counter Initial Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcivru](index.html) module"]
pub struct LCIVRU_SPEC;
impl crate::RegisterSpec for LCIVRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcivru::R](R) reader structure"]
impl crate::Readable for LCIVRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcivru::W](W) writer structure"]
impl crate::Writable for LCIVRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCIVRU to value 0"]
impl crate::Resettable for LCIVRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
