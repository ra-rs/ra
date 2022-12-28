#[doc = "Register `CCALCF` reader"]
pub struct R(crate::R<CCALCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCALCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCALCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCALCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCALCF` writer"]
pub struct W(crate::W<CCALCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCALCF_SPEC>;
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
impl From<crate::W<CCALCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCALCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC` reader - C-Cache Line Size"]
pub type CC_R = crate::FieldReader<u8, CC_A>;
#[doc = "C-Cache Line Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC_A {
    #[doc = "0: Prohibited"]
    _00 = 0,
    #[doc = "1: Cache line size 32 bytes"]
    _01 = 1,
    #[doc = "2: Cache line size 64 bytes"]
    _10 = 2,
    #[doc = "3: Prohibited"]
    _11 = 3,
}
impl From<CC_A> for u8 {
    #[inline(always)]
    fn from(variant: CC_A) -> Self {
        variant as _
    }
}
impl CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC_A {
        match self.bits {
            0 => CC_A::_00,
            1 => CC_A::_01,
            2 => CC_A::_10,
            3 => CC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CC_A::_11
    }
}
#[doc = "Field `CC` writer - C-Cache Line Size"]
pub type CC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCALCF_SPEC, u8, CC_A, 2, O>;
impl<'a, const O: u8> CC_W<'a, O> {
    #[doc = "Prohibited"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CC_A::_00)
    }
    #[doc = "Cache line size 32 bytes"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CC_A::_01)
    }
    #[doc = "Cache line size 64 bytes"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CC_A::_10)
    }
    #[doc = "Prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CC_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - C-Cache Line Size"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - C-Cache Line Size"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<0> {
        CC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "C-Cache Line Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccalcf](index.html) module"]
pub struct CCALCF_SPEC;
impl crate::RegisterSpec for CCALCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccalcf::R](R) reader structure"]
impl crate::Readable for CCALCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccalcf::W](W) writer structure"]
impl crate::Writable for CCALCF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCALCF to value 0x01"]
impl crate::Resettable for CCALCF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
