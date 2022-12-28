#[doc = "Register `GTADTRB` reader"]
pub struct R(crate::R<GTADTRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTADTRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTADTRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTADTRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTADTRB` writer"]
pub struct W(crate::W<GTADTRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTADTRB_SPEC>;
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
impl From<crate::W<GTADTRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTADTRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTADTRB` reader - A/D Converter Start Request Timing Register B"]
pub type GTADTRB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTADTRB` writer - A/D Converter Start Request Timing Register B"]
pub type GTADTRB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADTRB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A/D Converter Start Request Timing Register B"]
    #[inline(always)]
    pub fn gtadtrb(&self) -> GTADTRB_R {
        GTADTRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A/D Converter Start Request Timing Register B"]
    #[inline(always)]
    #[must_use]
    pub fn gtadtrb(&mut self) -> GTADTRB_W<0> {
        GTADTRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Start Request Timing Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtadtrb](index.html) module"]
pub struct GTADTRB_SPEC;
impl crate::RegisterSpec for GTADTRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtadtrb::R](R) reader structure"]
impl crate::Readable for GTADTRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtadtrb::W](W) writer structure"]
impl crate::Writable for GTADTRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTADTRB to value 0xffff_ffff"]
impl crate::Resettable for GTADTRB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
