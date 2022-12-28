#[doc = "Register `MMSFR` reader"]
pub struct R(crate::R<MMSFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMSFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMSFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMSFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMSFR` writer"]
pub struct W(crate::W<MMSFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMSFR_SPEC>;
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
impl From<crate::W<MMSFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMSFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMMIRADDR` reader - Specifies the memory mirror address.NOTE: A value cannot be set in the low-order 7 bits. These bits are fixed to 0."]
pub type MEMMIRADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEMMIRADDR` writer - Specifies the memory mirror address.NOTE: A value cannot be set in the low-order 7 bits. These bits are fixed to 0."]
pub type MEMMIRADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMSFR_SPEC, u16, u16, 16, O>;
#[doc = "MMSFR Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "219: Writing to the MEMMIRADDR bits are valid, when the KEY bits are written 0xDB."]
    _0X_DB = 219,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - MMSFR Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMSFR_SPEC, u8, KEY_AW, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing to the MEMMIRADDR bits are valid, when the KEY bits are written 0xDB."]
    #[inline(always)]
    pub fn _0x_db(self) -> &'a mut W {
        self.variant(KEY_AW::_0X_DB)
    }
}
impl R {
    #[doc = "Bits 7:22 - Specifies the memory mirror address.NOTE: A value cannot be set in the low-order 7 bits. These bits are fixed to 0."]
    #[inline(always)]
    pub fn memmiraddr(&self) -> MEMMIRADDR_R {
        MEMMIRADDR_R::new(((self.bits >> 7) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 7:22 - Specifies the memory mirror address.NOTE: A value cannot be set in the low-order 7 bits. These bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn memmiraddr(&mut self) -> MEMMIRADDR_W<7> {
        MEMMIRADDR_W::new(self)
    }
    #[doc = "Bits 24:31 - MMSFR Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MemMirror Special Function Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmsfr](index.html) module"]
pub struct MMSFR_SPEC;
impl crate::RegisterSpec for MMSFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmsfr::R](R) reader structure"]
impl crate::Readable for MMSFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmsfr::W](W) writer structure"]
impl crate::Writable for MMSFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMSFR to value 0"]
impl crate::Resettable for MMSFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
