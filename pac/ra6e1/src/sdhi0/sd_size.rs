#[doc = "Register `SD_SIZE` reader"]
pub struct R(crate::R<SD_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_SIZE` writer"]
pub struct W(crate::W<SD_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_SIZE_SPEC>;
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
impl From<crate::W<SD_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - Transfer Data Size Setting"]
pub type LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LEN` writer - Transfer Data Size Setting"]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_SIZE_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Transfer Data Size Setting"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transfer Data Size Setting"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_size](index.html) module"]
pub struct SD_SIZE_SPEC;
impl crate::RegisterSpec for SD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_size::R](R) reader structure"]
impl crate::Readable for SD_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_size::W](W) writer structure"]
impl crate::Writable for SD_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_SIZE to value 0x0200"]
impl crate::Resettable for SD_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
