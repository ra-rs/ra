#[doc = "Register `SCALCF` reader"]
pub struct R(crate::R<SCALCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCALCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCALCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCALCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCALCF` writer"]
pub struct W(crate::W<SCALCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCALCF_SPEC>;
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
impl From<crate::W<SCALCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCALCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS` reader - S-Cache Line Size"]
pub type CS_R = crate::FieldReader<u8, CS_A>;
#[doc = "S-Cache Line Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS_A {
    #[doc = "0: Prohibited"]
    _00 = 0,
    #[doc = "1: Cache line size 32 bytes"]
    _01 = 1,
    #[doc = "2: Cache line size 64 bytes"]
    _10 = 2,
    #[doc = "3: Prohibited"]
    _11 = 3,
}
impl From<CS_A> for u8 {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        variant as _
    }
}
impl CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_A {
        match self.bits {
            0 => CS_A::_00,
            1 => CS_A::_01,
            2 => CS_A::_10,
            3 => CS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CS_A::_11
    }
}
#[doc = "Field `CS` writer - S-Cache Line Size"]
pub type CS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SCALCF_SPEC, u8, CS_A, 2, O>;
impl<'a, const O: u8> CS_W<'a, O> {
    #[doc = "Prohibited"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CS_A::_00)
    }
    #[doc = "Cache line size 32 bytes"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CS_A::_01)
    }
    #[doc = "Cache line size 64 bytes"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CS_A::_10)
    }
    #[doc = "Prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - S-Cache Line Size"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - S-Cache Line Size"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<0> {
        CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S-Cache Line Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scalcf](index.html) module"]
pub struct SCALCF_SPEC;
impl crate::RegisterSpec for SCALCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scalcf::R](R) reader structure"]
impl crate::Readable for SCALCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scalcf::W](W) writer structure"]
impl crate::Writable for SCALCF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALCF to value 0x01"]
impl crate::Resettable for SCALCF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
