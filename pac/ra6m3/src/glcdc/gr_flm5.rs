#[doc = "Register `GR%s_FLM5` reader"]
pub struct R(crate::R<GR_FLM5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_FLM5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_FLM5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_FLM5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_FLM5` writer"]
pub struct W(crate::W<GR_FLM5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_FLM5_SPEC>;
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
impl From<crate::W<GR_FLM5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_FLM5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATANUM` reader - Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)"]
pub type DATANUM_R = crate::FieldReader<u16, DATANUM_A>;
#[doc = "Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DATANUM_A(u16);
impl From<DATANUM_A> for u16 {
    #[inline(always)]
    fn from(val: DATANUM_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `DATANUM` writer - Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)"]
pub type DATANUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GR_FLM5_SPEC, u16, DATANUM_A, 16, O>;
#[doc = "Field `LNNUM` reader - Number of lines per frame for accessing graphics data (frame buffer data)."]
pub type LNNUM_R = crate::FieldReader<u16, LNNUM_A>;
#[doc = "Number of lines per frame for accessing graphics data (frame buffer data).\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LNNUM_A(u16);
impl From<LNNUM_A> for u16 {
    #[inline(always)]
    fn from(val: LNNUM_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `LNNUM` writer - Number of lines per frame for accessing graphics data (frame buffer data)."]
pub type LNNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GR_FLM5_SPEC, u16, LNNUM_A, 11, O>;
impl R {
    #[doc = "Bits 0:15 - Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)"]
    #[inline(always)]
    pub fn datanum(&self) -> DATANUM_R {
        DATANUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - Number of lines per frame for accessing graphics data (frame buffer data)."]
    #[inline(always)]
    pub fn lnnum(&self) -> LNNUM_R {
        LNNUM_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)"]
    #[inline(always)]
    #[must_use]
    pub fn datanum(&mut self) -> DATANUM_W<0> {
        DATANUM_W::new(self)
    }
    #[doc = "Bits 16:26 - Number of lines per frame for accessing graphics data (frame buffer data)."]
    #[inline(always)]
    #[must_use]
    pub fn lnnum(&mut self) -> LNNUM_W<16> {
        LNNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Frame Buffer Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_flm5](index.html) module"]
pub struct GR_FLM5_SPEC;
impl crate::RegisterSpec for GR_FLM5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_flm5::R](R) reader structure"]
impl crate::Readable for GR_FLM5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_flm5::W](W) writer structure"]
impl crate::Writable for GR_FLM5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_FLM5 to value 0x000f_0000"]
impl crate::Resettable for GR_FLM5_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0000;
}
