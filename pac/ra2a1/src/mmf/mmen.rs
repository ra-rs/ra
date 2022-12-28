#[doc = "Register `MMEN` reader"]
pub struct R(crate::R<MMEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMEN` writer"]
pub struct W(crate::W<MMEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMEN_SPEC>;
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
impl From<crate::W<MMEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Memory Mirror Function Enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Memory Mirror Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "1: Memory Mirror Function is enabled."]
    _1 = 1,
    #[doc = "0: Memory Mirror Function is disabled."]
    _0 = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::_1,
            false => EN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
}
#[doc = "Field `EN` writer - Memory Mirror Function Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMEN_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Memory Mirror Function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
    }
    #[doc = "Memory Mirror Function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
}
#[doc = "MMEN Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "219: Writing to the EN bit is valid, when the KEY bits are written 0xDB."]
    _0X_DB = 219,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - MMEN Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMEN_SPEC, u8, KEY_AW, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing to the EN bit is valid, when the KEY bits are written 0xDB."]
    #[inline(always)]
    pub fn _0x_db(self) -> &'a mut W {
        self.variant(KEY_AW::_0X_DB)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Mirror Function Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Mirror Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 24:31 - MMEN Key Code"]
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
#[doc = "MemMirror Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmen](index.html) module"]
pub struct MMEN_SPEC;
impl crate::RegisterSpec for MMEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmen::R](R) reader structure"]
impl crate::Readable for MMEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmen::W](W) writer structure"]
impl crate::Writable for MMEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMEN to value 0"]
impl crate::Resettable for MMEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
