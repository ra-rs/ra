#[doc = "Register `DMCRA` reader"]
pub struct R(crate::R<DMCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMCRA` writer"]
pub struct W(crate::W<DMCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMCRA_SPEC>;
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
impl From<crate::W<DMCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMCRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMCRAL` reader - Lower bits of transfer count"]
pub type DMCRAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMCRAL` writer - Lower bits of transfer count"]
pub type DMCRAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCRA_SPEC, u16, u16, 16, O>;
#[doc = "Field `DMCRAH` reader - Upper bits of transfer count"]
pub type DMCRAH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMCRAH` writer - Upper bits of transfer count"]
pub type DMCRAH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCRA_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:15 - Lower bits of transfer count"]
    #[inline(always)]
    pub fn dmcral(&self) -> DMCRAL_R {
        DMCRAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Upper bits of transfer count"]
    #[inline(always)]
    pub fn dmcrah(&self) -> DMCRAH_R {
        DMCRAH_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower bits of transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn dmcral(&mut self) -> DMCRAL_W<0> {
        DMCRAL_W::new(self)
    }
    #[doc = "Bits 16:25 - Upper bits of transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn dmcrah(&mut self) -> DMCRAH_W<16> {
        DMCRAH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Transfer Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmcra](index.html) module"]
pub struct DMCRA_SPEC;
impl crate::RegisterSpec for DMCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmcra::R](R) reader structure"]
impl crate::Readable for DMCRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmcra::W](W) writer structure"]
impl crate::Writable for DMCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCRA to value 0"]
impl crate::Resettable for DMCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
