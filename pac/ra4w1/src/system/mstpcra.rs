#[doc = "Register `MSTPCRA` reader"]
pub struct R(crate::R<MSTPCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRA` writer"]
pub struct W(crate::W<MSTPCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRA_SPEC>;
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
impl From<crate::W<MSTPCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPA0` reader - RAM0 Module Stop"]
pub type MSTPA0_R = crate::BitReader<MSTPA0_A>;
#[doc = "RAM0 Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA0_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPA0_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA0_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPA0_A {
        match self.bits {
            false => MSTPA0_A::_0,
            true => MSTPA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA0_A::_1
    }
}
#[doc = "Field `MSTPA0` writer - RAM0 Module Stop"]
pub type MSTPA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRA_SPEC, MSTPA0_A, O>;
impl<'a, const O: u8> MSTPA0_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPA0_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPA0_A::_1)
    }
}
#[doc = "Field `MSTPA6` reader - ECCRAM Module Stop"]
pub type MSTPA6_R = crate::BitReader<MSTPA6_A>;
#[doc = "ECCRAM Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA6_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPA6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA6_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPA6_A {
        match self.bits {
            false => MSTPA6_A::_0,
            true => MSTPA6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA6_A::_1
    }
}
#[doc = "Field `MSTPA6` writer - ECCRAM Module Stop"]
pub type MSTPA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRA_SPEC, MSTPA6_A, O>;
impl<'a, const O: u8> MSTPA6_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPA6_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPA6_A::_1)
    }
}
#[doc = "Field `MSTPA22` reader - DMA Controller/Data Transfer Controller Module Stop"]
pub type MSTPA22_R = crate::BitReader<MSTPA22_A>;
#[doc = "DMA Controller/Data Transfer Controller Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA22_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPA22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA22_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPA22_A {
        match self.bits {
            false => MSTPA22_A::_0,
            true => MSTPA22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA22_A::_1
    }
}
#[doc = "Field `MSTPA22` writer - DMA Controller/Data Transfer Controller Module Stop"]
pub type MSTPA22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRA_SPEC, MSTPA22_A, O>;
impl<'a, const O: u8> MSTPA22_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPA22_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPA22_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RAM0 Module Stop"]
    #[inline(always)]
    pub fn mstpa0(&self) -> MSTPA0_R {
        MSTPA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - ECCRAM Module Stop"]
    #[inline(always)]
    pub fn mstpa6(&self) -> MSTPA6_R {
        MSTPA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    pub fn mstpa22(&self) -> MSTPA22_R {
        MSTPA22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpa0(&mut self) -> MSTPA0_W<0> {
        MSTPA0_W::new(self)
    }
    #[doc = "Bit 6 - ECCRAM Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpa6(&mut self) -> MSTPA6_W<6> {
        MSTPA6_W::new(self)
    }
    #[doc = "Bit 22 - DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpa22(&mut self) -> MSTPA22_W<22> {
        MSTPA22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcra](index.html) module"]
pub struct MSTPCRA_SPEC;
impl crate::RegisterSpec for MSTPCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcra::R](R) reader structure"]
impl crate::Readable for MSTPCRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcra::W](W) writer structure"]
impl crate::Writable for MSTPCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRA to value 0xffbf_ffbe"]
impl crate::Resettable for MSTPCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffbf_ffbe;
}
