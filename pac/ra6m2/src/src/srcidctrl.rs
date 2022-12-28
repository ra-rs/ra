#[doc = "Register `SRCIDCTRL` reader"]
pub struct R(crate::R<SRCIDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCIDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCIDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCIDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCIDCTRL` writer"]
pub struct W(crate::W<SRCIDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCIDCTRL_SPEC>;
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
impl From<crate::W<SRCIDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCIDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFTRG` reader - Input FIFO Data Triggering Number"]
pub type IFTRG_R = crate::FieldReader<u8, IFTRG_A>;
#[doc = "Input FIFO Data Triggering Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFTRG_A {
    #[doc = "0: 0"]
    _00 = 0,
    #[doc = "1: 2"]
    _01 = 1,
    #[doc = "2: 4"]
    _10 = 2,
    #[doc = "3: 6"]
    _11 = 3,
}
impl From<IFTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: IFTRG_A) -> Self {
        variant as _
    }
}
impl IFTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFTRG_A {
        match self.bits {
            0 => IFTRG_A::_00,
            1 => IFTRG_A::_01,
            2 => IFTRG_A::_10,
            3 => IFTRG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IFTRG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IFTRG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IFTRG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IFTRG_A::_11
    }
}
#[doc = "Field `IFTRG` writer - Input FIFO Data Triggering Number"]
pub type IFTRG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SRCIDCTRL_SPEC, u8, IFTRG_A, 2, O>;
impl<'a, const O: u8> IFTRG_W<'a, O> {
    #[doc = "0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IFTRG_A::_00)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IFTRG_A::_01)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IFTRG_A::_10)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IFTRG_A::_11)
    }
}
#[doc = "Field `IEN` reader - Input FIFO Empty Interrupt Enable"]
pub type IEN_R = crate::BitReader<IEN_A>;
#[doc = "Input FIFO Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN_A {
    #[doc = "0: Input FIFO empty interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Input FIFO empty interrupt is enabled."]
    _1 = 1,
}
impl From<IEN_A> for bool {
    #[inline(always)]
    fn from(variant: IEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN_A {
        match self.bits {
            false => IEN_A::_0,
            true => IEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEN_A::_1
    }
}
#[doc = "Field `IEN` writer - Input FIFO Empty Interrupt Enable"]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCIDCTRL_SPEC, IEN_A, O>;
impl<'a, const O: u8> IEN_W<'a, O> {
    #[doc = "Input FIFO empty interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEN_A::_0)
    }
    #[doc = "Input FIFO empty interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEN_A::_1)
    }
}
#[doc = "Field `IED` reader - Input Data Endian"]
pub type IED_R = crate::BitReader<IED_A>;
#[doc = "Input Data Endian\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IED_A {
    #[doc = "0: Endian formats 1 are the same between the CPU and input data."]
    _0 = 0,
    #[doc = "1: Endian formats 1 are different between the CPU and input data."]
    _1 = 1,
}
impl From<IED_A> for bool {
    #[inline(always)]
    fn from(variant: IED_A) -> Self {
        variant as u8 != 0
    }
}
impl IED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IED_A {
        match self.bits {
            false => IED_A::_0,
            true => IED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IED_A::_1
    }
}
#[doc = "Field `IED` writer - Input Data Endian"]
pub type IED_W<'a, const O: u8> = crate::BitWriter<'a, u16, SRCIDCTRL_SPEC, IED_A, O>;
impl<'a, const O: u8> IED_W<'a, O> {
    #[doc = "Endian formats 1 are the same between the CPU and input data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IED_A::_0)
    }
    #[doc = "Endian formats 1 are different between the CPU and input data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IED_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Input FIFO Data Triggering Number"]
    #[inline(always)]
    pub fn iftrg(&self) -> IFTRG_R {
        IFTRG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Input FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input Data Endian"]
    #[inline(always)]
    pub fn ied(&self) -> IED_R {
        IED_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input FIFO Data Triggering Number"]
    #[inline(always)]
    #[must_use]
    pub fn iftrg(&mut self) -> IFTRG_W<0> {
        IFTRG_W::new(self)
    }
    #[doc = "Bit 8 - Input FIFO Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<8> {
        IEN_W::new(self)
    }
    #[doc = "Bit 9 - Input Data Endian"]
    #[inline(always)]
    #[must_use]
    pub fn ied(&mut self) -> IED_W<9> {
        IED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Data Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcidctrl](index.html) module"]
pub struct SRCIDCTRL_SPEC;
impl crate::RegisterSpec for SRCIDCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [srcidctrl::R](R) reader structure"]
impl crate::Readable for SRCIDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcidctrl::W](W) writer structure"]
impl crate::Writable for SRCIDCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCIDCTRL to value 0"]
impl crate::Resettable for SRCIDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
