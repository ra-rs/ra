#[doc = "Register `OUT_BRIGHT2` reader"]
pub struct R(crate::R<OUT_BRIGHT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_BRIGHT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_BRIGHT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_BRIGHT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_BRIGHT2` writer"]
pub struct W(crate::W<OUT_BRIGHT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_BRIGHT2_SPEC>;
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
impl From<crate::W<OUT_BRIGHT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_BRIGHT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRTR` reader - Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer"]
pub type BRTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRTR` writer - Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer"]
pub type BRTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_BRIGHT2_SPEC, u16, u16, 10, O>;
#[doc = "Field `BRTB` reader - Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer"]
pub type BRTB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRTB` writer - Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer"]
pub type BRTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_BRIGHT2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    pub fn brtr(&self) -> BRTR_R {
        BRTR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    pub fn brtb(&self) -> BRTB_R {
        BRTB_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    #[must_use]
    pub fn brtr(&mut self) -> BRTR_W<0> {
        BRTR_W::new(self)
    }
    #[doc = "Bits 16:25 - Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    #[must_use]
    pub fn brtb(&mut self) -> BRTB_W<16> {
        BRTB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Control Block Brightness Correction Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_bright2](index.html) module"]
pub struct OUT_BRIGHT2_SPEC;
impl crate::RegisterSpec for OUT_BRIGHT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_bright2::R](R) reader structure"]
impl crate::Readable for OUT_BRIGHT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_bright2::W](W) writer structure"]
impl crate::Writable for OUT_BRIGHT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_BRIGHT2 to value 0"]
impl crate::Resettable for OUT_BRIGHT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
