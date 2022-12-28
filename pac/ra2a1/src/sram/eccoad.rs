#[doc = "Register `ECCOAD` reader"]
pub struct R(crate::R<ECCOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCOAD` writer"]
pub struct W(crate::W<ECCOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCOAD_SPEC>;
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
impl From<crate::W<ECCOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OAD` reader - Operation after Detection"]
pub type OAD_R = crate::BitReader<OAD_A>;
#[doc = "Operation after Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    #[doc = "0: Non maskable interrupt."]
    _0 = 0,
    #[doc = "1: Internal reset."]
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
pub type OAD_W<'a, const O: u8> = crate::BitWriter<'a, u8, ECCOAD_SPEC, OAD_A, O>;
impl<'a, const O: u8> OAD_W<'a, O> {
    #[doc = "Non maskable interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAD_A::_0)
    }
    #[doc = "Internal reset."]
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM ECC Error Operation After Detection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccoad](index.html) module"]
pub struct ECCOAD_SPEC;
impl crate::RegisterSpec for ECCOAD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eccoad::R](R) reader structure"]
impl crate::Readable for ECCOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccoad::W](W) writer structure"]
impl crate::Writable for ECCOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCOAD to value 0"]
impl crate::Resettable for ECCOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
