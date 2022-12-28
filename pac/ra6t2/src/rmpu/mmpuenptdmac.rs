#[doc = "Register `MMPUENPTDMAC` reader"]
pub struct R(crate::R<MMPUENPTDMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUENPTDMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUENPTDMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUENPTDMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUENPTDMAC` writer"]
pub struct W(crate::W<MMPUENPTDMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUENPTDMAC_SPEC>;
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
impl From<crate::W<MMPUENPTDMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUENPTDMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTECT` reader - Protection of register"]
pub type PROTECT_R = crate::BitReader<PROTECT_A>;
#[doc = "Protection of register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROTECT_A {
    #[doc = "0: MMPUENDMAC register writes are possible."]
    _0 = 0,
    #[doc = "1: MMPUENDMAC register writes are protected. Read is possible."]
    _1 = 1,
}
impl From<PROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: PROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTECT_A {
        match self.bits {
            false => PROTECT_A::_0,
            true => PROTECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROTECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROTECT_A::_1
    }
}
#[doc = "Field `PROTECT` writer - Protection of register"]
pub type PROTECT_W<'a, const O: u8> = crate::BitWriter<'a, u16, MMPUENPTDMAC_SPEC, PROTECT_A, O>;
impl<'a, const O: u8> PROTECT_W<'a, O> {
    #[doc = "MMPUENDMAC register writes are possible."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROTECT_A::_0)
    }
    #[doc = "MMPUENDMAC register writes are protected. Read is possible."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROTECT_A::_1)
    }
}
#[doc = "Field `KEY` writer - These bits enable or disable writes to the PROTECT bit."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MMPUENPTDMAC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Protection of register"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protection of register"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> PROTECT_W<0> {
        PROTECT_W::new(self)
    }
    #[doc = "Bits 8:15 - These bits enable or disable writes to the PROTECT bit."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMPU Enable Protect Register for DMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpuenptdmac](index.html) module"]
pub struct MMPUENPTDMAC_SPEC;
impl crate::RegisterSpec for MMPUENPTDMAC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mmpuenptdmac::R](R) reader structure"]
impl crate::Readable for MMPUENPTDMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpuenptdmac::W](W) writer structure"]
impl crate::Writable for MMPUENPTDMAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUENPTDMAC to value 0"]
impl crate::Resettable for MMPUENPTDMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
