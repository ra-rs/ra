#[doc = "Register `RDLAR` reader"]
pub struct R(crate::R<RDLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDLAR` writer"]
pub struct W(crate::W<RDLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDLAR_SPEC>;
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
impl From<crate::W<RDLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDLAR` reader - The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\\[1:0\\]
bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b"]
pub type RDLAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RDLAR` writer - The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\\[1:0\\]
bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b"]
pub type RDLAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RDLAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\\[1:0\\]
bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b"]
    #[inline(always)]
    pub fn rdlar(&self) -> RDLAR_R {
        RDLAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\\[1:0\\]
bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b"]
    #[inline(always)]
    #[must_use]
    pub fn rdlar(&mut self) -> RDLAR_W<0> {
        RDLAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Descriptor List Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdlar](index.html) module"]
pub struct RDLAR_SPEC;
impl crate::RegisterSpec for RDLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdlar::R](R) reader structure"]
impl crate::Readable for RDLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdlar::W](W) writer structure"]
impl crate::Writable for RDLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDLAR to value 0"]
impl crate::Resettable for RDLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
