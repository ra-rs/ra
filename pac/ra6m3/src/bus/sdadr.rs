#[doc = "Register `SDADR` reader"]
pub struct R(crate::R<SDADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDADR` writer"]
pub struct W(crate::W<SDADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDADR_SPEC>;
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
impl From<crate::W<SDADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MXC` reader - Address Multiplex Select"]
pub type MXC_R = crate::FieldReader<u8, MXC_A>;
#[doc = "Address Multiplex Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MXC_A {
    #[doc = "0: 8-bit shift"]
    _00 = 0,
    #[doc = "1: 9-bit shift"]
    _01 = 1,
    #[doc = "2: 10-bit shift"]
    _10 = 2,
    #[doc = "3: 11-bit shift"]
    _11 = 3,
}
impl From<MXC_A> for u8 {
    #[inline(always)]
    fn from(variant: MXC_A) -> Self {
        variant as _
    }
}
impl MXC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MXC_A {
        match self.bits {
            0 => MXC_A::_00,
            1 => MXC_A::_01,
            2 => MXC_A::_10,
            3 => MXC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MXC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MXC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MXC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MXC_A::_11
    }
}
#[doc = "Field `MXC` writer - Address Multiplex Select"]
pub type MXC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SDADR_SPEC, u8, MXC_A, 2, O>;
impl<'a, const O: u8> MXC_W<'a, O> {
    #[doc = "8-bit shift"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MXC_A::_00)
    }
    #[doc = "9-bit shift"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MXC_A::_01)
    }
    #[doc = "10-bit shift"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MXC_A::_10)
    }
    #[doc = "11-bit shift"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MXC_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Address Multiplex Select"]
    #[inline(always)]
    pub fn mxc(&self) -> MXC_R {
        MXC_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Address Multiplex Select"]
    #[inline(always)]
    #[must_use]
    pub fn mxc(&mut self) -> MXC_W<0> {
        MXC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdadr](index.html) module"]
pub struct SDADR_SPEC;
impl crate::RegisterSpec for SDADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sdadr::R](R) reader structure"]
impl crate::Readable for SDADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdadr::W](W) writer structure"]
impl crate::Writable for SDADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDADR to value 0"]
impl crate::Resettable for SDADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
