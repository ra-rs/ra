#[doc = "Register `CAPOAD` reader"]
pub struct R(crate::R<CAPOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPOAD` writer"]
pub struct W(crate::W<CAPOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPOAD_SPEC>;
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
impl From<crate::W<CAPOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OAD` reader - Operation after Detection"]
pub type OAD_R = crate::BitReader<OAD_A>;
#[doc = "Operation after Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    #[doc = "0: Non-maskable interrupt"]
    _0 = 0,
    #[doc = "1: Reset"]
    _1 = 1,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAD_A {
        match self.bits {
            false => OAD_A::_0,
            true => OAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
}
#[doc = "Field `OAD` writer - Operation after Detection"]
pub type OAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPOAD_SPEC, OAD_A, O>;
impl<'a, const O: u8> OAD_W<'a, O> {
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAD_A::_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OAD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Operation after Detection"]
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation after Detection"]
    #[inline(always)]
    #[must_use]
    pub fn oad(&mut self) -> OAD_W<0> {
        OAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Parity Error Operation After Detection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capoad](index.html) module"]
pub struct CAPOAD_SPEC;
impl crate::RegisterSpec for CAPOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capoad::R](R) reader structure"]
impl crate::Readable for CAPOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capoad::W](W) writer structure"]
impl crate::Writable for CAPOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPOAD to value 0"]
impl crate::Resettable for CAPOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
