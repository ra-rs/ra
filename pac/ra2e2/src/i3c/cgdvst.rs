#[doc = "Register `CGDVST` reader"]
pub struct R(crate::R<CGDVST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGDVST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGDVST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGDVST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGDVST` writer"]
pub struct W(crate::W<CGDVST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGDVST_SPEC>;
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
impl From<crate::W<CGDVST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGDVST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PNDINT` reader - Pending Interrupt"]
pub type PNDINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PNDINT` writer - Pending Interrupt"]
pub type PNDINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CGDVST_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRTE` reader - Protocol Error"]
pub type PRTE_R = crate::BitReader<PRTE_A>;
#[doc = "Protocol Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRTE_A {
    #[doc = "0: The Slave has not detected a protocol error since the last Status read."]
    _0 = 0,
    #[doc = "1: The Slave has detected a protocol error since the last Status read."]
    _1 = 1,
}
impl From<PRTE_A> for bool {
    #[inline(always)]
    fn from(variant: PRTE_A) -> Self {
        variant as u8 != 0
    }
}
impl PRTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTE_A {
        match self.bits {
            false => PRTE_A::_0,
            true => PRTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRTE_A::_1
    }
}
#[doc = "Field `PRTE` writer - Protocol Error"]
pub type PRTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGDVST_SPEC, PRTE_A, O>;
impl<'a, const O: u8> PRTE_W<'a, O> {
    #[doc = "The Slave has not detected a protocol error since the last Status read."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRTE_A::_0)
    }
    #[doc = "The Slave has detected a protocol error since the last Status read."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRTE_A::_1)
    }
}
#[doc = "Field `ACTMD` reader - Slave Device’s current Activity Mode"]
pub type ACTMD_R = crate::FieldReader<u8, ACTMD_A>;
#[doc = "Slave Device’s current Activity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTMD_A {
    #[doc = "0: Activity Mode 0"]
    _00 = 0,
    #[doc = "1: Activity Mode 1"]
    _01 = 1,
    #[doc = "2: Activity Mode 2"]
    _10 = 2,
    #[doc = "3: Activity Mode 3"]
    _11 = 3,
}
impl From<ACTMD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTMD_A) -> Self {
        variant as _
    }
}
impl ACTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTMD_A {
        match self.bits {
            0 => ACTMD_A::_00,
            1 => ACTMD_A::_01,
            2 => ACTMD_A::_10,
            3 => ACTMD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ACTMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ACTMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ACTMD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ACTMD_A::_11
    }
}
#[doc = "Field `ACTMD` writer - Slave Device’s current Activity Mode"]
pub type ACTMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CGDVST_SPEC, u8, ACTMD_A, 2, O>;
impl<'a, const O: u8> ACTMD_W<'a, O> {
    #[doc = "Activity Mode 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACTMD_A::_00)
    }
    #[doc = "Activity Mode 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACTMD_A::_01)
    }
    #[doc = "Activity Mode 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ACTMD_A::_10)
    }
    #[doc = "Activity Mode 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ACTMD_A::_11)
    }
}
#[doc = "Field `VDRSV` reader - Vendor Reserved"]
pub type VDRSV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDRSV` writer - Vendor Reserved"]
pub type VDRSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CGDVST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - Pending Interrupt"]
    #[inline(always)]
    pub fn pndint(&self) -> PNDINT_R {
        PNDINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Protocol Error"]
    #[inline(always)]
    pub fn prte(&self) -> PRTE_R {
        PRTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Slave Device’s current Activity Mode"]
    #[inline(always)]
    pub fn actmd(&self) -> ACTMD_R {
        ACTMD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Reserved"]
    #[inline(always)]
    pub fn vdrsv(&self) -> VDRSV_R {
        VDRSV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pndint(&mut self) -> PNDINT_W<0> {
        PNDINT_W::new(self)
    }
    #[doc = "Bit 5 - Protocol Error"]
    #[inline(always)]
    #[must_use]
    pub fn prte(&mut self) -> PRTE_W<5> {
        PRTE_W::new(self)
    }
    #[doc = "Bits 6:7 - Slave Device’s current Activity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn actmd(&mut self) -> ACTMD_W<6> {
        ACTMD_W::new(self)
    }
    #[doc = "Bits 8:15 - Vendor Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn vdrsv(&mut self) -> VDRSV_W<8> {
        VDRSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Get Device Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgdvst](index.html) module"]
pub struct CGDVST_SPEC;
impl crate::RegisterSpec for CGDVST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgdvst::R](R) reader structure"]
impl crate::Readable for CGDVST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgdvst::W](W) writer structure"]
impl crate::Writable for CGDVST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGDVST to value 0"]
impl crate::Resettable for CGDVST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
