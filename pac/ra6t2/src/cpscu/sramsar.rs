#[doc = "Register `SRAMSAR` reader"]
pub struct R(crate::R<SRAMSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMSAR` writer"]
pub struct W(crate::W<SRAMSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMSAR_SPEC>;
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
impl From<crate::W<SRAMSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAMSA0` reader - Security attributes of registers for SRAM Protection"]
pub type SRAMSA0_R = crate::BitReader<SRAMSA0_A>;
#[doc = "Security attributes of registers for SRAM Protection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMSA0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-Secure"]
    _1 = 1,
}
impl From<SRAMSA0_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMSA0_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAMSA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMSA0_A {
        match self.bits {
            false => SRAMSA0_A::_0,
            true => SRAMSA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAMSA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAMSA0_A::_1
    }
}
#[doc = "Field `SRAMSA0` writer - Security attributes of registers for SRAM Protection"]
pub type SRAMSA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMSAR_SPEC, SRAMSA0_A, O>;
impl<'a, const O: u8> SRAMSA0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAMSA0_A::_0)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAMSA0_A::_1)
    }
}
#[doc = "Field `SRAMSA2` reader - Security attributes of registers for ECC Relation"]
pub type SRAMSA2_R = crate::BitReader<SRAMSA2_A>;
#[doc = "Security attributes of registers for ECC Relation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMSA2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-Secure"]
    _1 = 1,
}
impl From<SRAMSA2_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMSA2_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAMSA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMSA2_A {
        match self.bits {
            false => SRAMSA2_A::_0,
            true => SRAMSA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAMSA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAMSA2_A::_1
    }
}
#[doc = "Field `SRAMSA2` writer - Security attributes of registers for ECC Relation"]
pub type SRAMSA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMSAR_SPEC, SRAMSA2_A, O>;
impl<'a, const O: u8> SRAMSA2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAMSA2_A::_0)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAMSA2_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for SRAM Protection"]
    #[inline(always)]
    pub fn sramsa0(&self) -> SRAMSA0_R {
        SRAMSA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Security attributes of registers for ECC Relation"]
    #[inline(always)]
    pub fn sramsa2(&self) -> SRAMSA2_R {
        SRAMSA2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for SRAM Protection"]
    #[inline(always)]
    #[must_use]
    pub fn sramsa0(&mut self) -> SRAMSA0_W<0> {
        SRAMSA0_W::new(self)
    }
    #[doc = "Bit 2 - Security attributes of registers for ECC Relation"]
    #[inline(always)]
    #[must_use]
    pub fn sramsa2(&mut self) -> SRAMSA2_W<2> {
        SRAMSA2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramsar](index.html) module"]
pub struct SRAMSAR_SPEC;
impl crate::RegisterSpec for SRAMSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sramsar::R](R) reader structure"]
impl crate::Readable for SRAMSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramsar::W](W) writer structure"]
impl crate::Writable for SRAMSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMSAR to value 0xffff_ffff"]
impl crate::Resettable for SRAMSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
